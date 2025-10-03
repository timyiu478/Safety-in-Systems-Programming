
1. Debugger: disable handling of ctrl+c in this process (so that ctrl+c only gets delivered to child.
1. If a process is being traced under ptrace, all signals will cause it to temporarily stop instead, as if it were sent SIGSTOP. This is useful for debugging: if a program segfaults but is being traced under ptrace, the program will stop instead of terminating so that you can get a backtrace and inspect its memory.
