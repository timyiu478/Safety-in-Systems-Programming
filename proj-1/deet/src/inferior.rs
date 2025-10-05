use nix::sys::ptrace;
use nix::sys::signal;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::Pid;
use std::mem::size_of;
use std::process::{Child, Command};
use std::os::unix::process::CommandExt;
use std::collections::HashMap;
use crate::dwarf_data::DwarfData;

pub enum Status {
    /// Indicates inferior stopped. Contains the signal that stopped the process, as well as the
    /// current instruction pointer that it is stopped at.
    Stopped(signal::Signal, usize),

    /// Indicates inferior exited normally. Contains the exit status code.
    Exited(i32),

    /// Indicates the inferior exited due to a signal. Contains the signal that killed the
    /// process.
    Signaled(signal::Signal),
}

/// This function calls ptrace with PTRACE_TRACEME to enable debugging on a process. You should use
/// pre_exec with Command to call this in the child process.
fn child_traceme() -> Result<(), std::io::Error> {
    ptrace::traceme().or(Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "ptrace TRACEME failed",
    )))
}

fn align_addr_to_word(addr: usize) -> usize {
    addr & (-(size_of::<usize>() as isize) as usize)
}

#[derive(Clone)]
struct Breakpoint {
    addr: usize,
    orig_byte: u8,
}

pub struct Inferior {
    child: Child,
    breakpoints: HashMap<usize, Breakpoint>,
}

impl Inferior {
    /// Attempts to start a new inferior process. Returns Some(Inferior) if successful, or None if
    /// an error is encountered.
    pub fn new(target: &str, args: &Vec<String>, breakpoints: &Vec<usize>) -> Option<Inferior> {
        
        let mut binding = Command::new(target);
        let mut cmd = binding.args(args);
        unsafe {
            cmd = cmd.pre_exec(|| child_traceme());
        }
        let ch = cmd.spawn().ok()?;

        // verify that it stops with signal SIGTRAP 
        // https://linux.die.net/man/2/waitpid
        let pid = Pid::from_raw(ch.id() as i32); 
        waitpid(pid, Some(WaitPidFlag::WSTOPPED)).ok()?;

        let mut inf = Inferior { child: ch, breakpoints: HashMap::new() };

        // set all the breakpoints that were passed in
        for addr in breakpoints {
            let orig_byte = inf.write_byte(*addr, 0xcc).expect("Failed to set breakpoint");
            inf.breakpoints.insert(*addr, Breakpoint { addr: *addr, orig_byte: orig_byte as u8 });
        }

        Some(inf)
    }

    /// Returns the pid of this inferior.
    pub fn pid(&self) -> Pid {
        nix::unistd::Pid::from_raw(self.child.id() as i32)
    }

    /// Calls waitpid on this inferior and returns a Status to indicate the state of the process
    /// after the waitpid call.
    pub fn wait(&self, options: Option<WaitPidFlag>) -> Result<Status, nix::Error> {
        Ok(match waitpid(self.pid(), options)? {
            WaitStatus::Exited(_pid, exit_code) => Status::Exited(exit_code),
            WaitStatus::Signaled(_pid, signal, _core_dumped) => Status::Signaled(signal),
            WaitStatus::Stopped(_pid, signal) => {
                let regs = ptrace::getregs(self.pid())?;
                Status::Stopped(signal, regs.rip as usize)
            }
            other => panic!("waitpid returned unexpected status: {:?}", other),
        })
    }

    pub fn print_backtrace(&self, debug_data: &DwarfData) -> Result<(), nix::Error> {
        let regs = ptrace::getregs(self.pid())?;
        let mut rip = regs.rip as usize;
        let mut rbp = regs.rbp as usize;

        loop {
            let line_number = debug_data.get_line_from_addr(rip).expect("unable to find line number from rip");
            let function_name = debug_data.get_function_from_addr(rip).expect("unable to find function name from rip");
            println!("{} ({})", function_name, line_number);
            if function_name == "main" {
                break;
            }
            // read next rip and rbp from stack
            rip = ptrace::read(self.pid(), (rbp + 8) as ptrace::AddressType)? as usize;
            rbp = ptrace::read(self.pid(), rbp as ptrace::AddressType)? as usize;
        }
        return Ok(());
    }
    
    pub fn print_stopped_location(&self, debug_data: &DwarfData) -> Result<(), nix::Error> {
        let regs = ptrace::getregs(self.pid())?;
        let rip = regs.rip as usize;
        let line_number = debug_data.get_line_from_addr(rip).expect("unable to find line number from rip");
        println!("Stopped at {}", line_number);
        return Ok(());
    }

    // Continues the inferior process.
    pub fn cont(&mut self) -> Result<Status, nix::Error> {

        let mut regs = ptrace::getregs(self.pid())?;
        let rip = regs.rip as usize;
        if let Some(bp) = self.breakpoints.get(& (rip - 1)) {
            let addr = bp.addr;
            // we are at a breakpoint, need to step over it
            // restore original byte
            self.write_byte(bp.addr, bp.orig_byte).expect("Failed to restore original byte at breakpoint");
            // set rip back to original instruction
            regs.rip = addr as u64;
            ptrace::setregs(self.pid(), regs).expect("Failed to set registers");
            // single step
            let _ = ptrace::step(self.pid(), None);
            let _ = self.wait(None);

            // reset the breakpoint
            let _ = self.write_byte(addr, 0xcc).expect("Failed to set breakpoint");
        }

        let _ = ptrace::cont(self.pid(), None);
        self.wait(None)
    }

    pub fn kill(&mut self) -> Result<Status, nix::Error> {
        let _ = self.child.kill();
        self.wait(None)
    }

    pub fn set_breakpoint(&mut self, addr: usize) -> Result<u8, nix::Error> {
        let orig_byte = self.write_byte(addr, 0xcc).expect("Failed to set breakpoint");
        self.breakpoints.insert(addr, Breakpoint { addr: addr, orig_byte: orig_byte as u8 });
        Ok(orig_byte as u8)
    }

    pub fn get_breakpoints_count(&mut self) -> usize {
        self.breakpoints.len()
    }

    fn write_byte(&mut self, addr: usize, val: u8) -> Result<u8, nix::Error> {
        let aligned_addr = align_addr_to_word(addr);
        let byte_offset = addr - aligned_addr;
        let word = ptrace::read(self.pid(), aligned_addr as ptrace::AddressType)? as u64;
        let orig_byte = (word >> 8 * byte_offset) & 0xff;
        let masked_word = word & !(0xff << 8 * byte_offset);
        let updated_word = masked_word | ((val as u64) << 8 * byte_offset);
        unsafe {
            ptrace::write(
                self.pid(),
                aligned_addr as ptrace::AddressType,
                updated_word as *mut std::ffi::c_void,
            )?;
        }
        Ok(orig_byte as u8)
    }
}
