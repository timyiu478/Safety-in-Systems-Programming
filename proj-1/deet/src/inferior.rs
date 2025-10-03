use nix::sys::ptrace;
use nix::sys::signal;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::Pid;
use std::process::{Child, Command};
use std::os::unix::process::CommandExt;

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

pub struct Inferior {
    child: Child,
}

impl Inferior {
    /// Attempts to start a new inferior process. Returns Some(Inferior) if successful, or None if
    /// an error is encountered.
    pub fn new(target: &str, args: &Vec<String>) -> Option<Inferior> {
        
        let mut binding = Command::new(target);
        let mut cmd = binding.args(args);
        unsafe {
            cmd = cmd.pre_exec(|| child_traceme());
        }
        let ch = cmd.spawn().ok()?;

        //verify that it stops with signal SIGTRAP 
        // https://linux.die.net/man/2/waitpid
        waitpid(Pid::from_raw(ch.id() as i32), Some(WaitPidFlag::WSTOPPED)).ok()?;

        Some(Inferior { child: ch })
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
        // print out the value of the %rip register.
        let regs = ptrace::getregs(self.pid())?;
        let rip = regs.rip as usize;
        // TODO
        // DwarfData::get_addr_for_line(
        // DwarfData::get_function_from_addr

        return Ok(());
    }

    // Continues the inferior process.
    pub fn cont(&self) -> Result<Status, nix::Error> {
        let _ = ptrace::cont(self.pid(), None);
        self.wait(None)
    }

    pub fn kill(&mut self) -> Result<Status, nix::Error> {
        let _ = self.child.kill();
        self.wait(None)
    }
}
