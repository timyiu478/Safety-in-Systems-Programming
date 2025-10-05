# Simple Debugger

This project will give us practice with multiprocessing in Rust, and will give us a better sense of how processes are managed by the operating system as well as how `ptrace` can be used to circumvent process boundaries.

## Features

1. run, stop, continue, kill the traced process
1. print a stack trace for a paused program
1. set the breakpoints

## Commands

1. `r/run`: start the traced program, kill any existing traced program first
1. `c/cont`: continue the traced program if it is stopped 
1. `q/quit`: exit the debugger, killing any traced program first
1. `Ctrl-C`: stop the traced program
1. `bt/back/backtrace`: print a stack trace for the traced program if it is stopped
1. `b/break`: set a breakpoint on the address if no program is traced

## Example Run

The source code of `samples/sleepy_print.c` that we will be debugging:

```c
1	#include <stdio.h>
2	#include <stdlib.h>
3	#include <unistd.h>
4	
5	int main(int argc, char *argv[]) {
6	    unsigned long num_seconds;
7	    if (argc != 2 || (num_seconds = strtoul(argv[1], NULL, 10)) == 0) {
8	        fprintf(stderr, "Usage: %s <seconds to sleep>\n", argv[0]);
9	        exit(1);
10	    }
11	    for (unsigned long i = 0; i < num_seconds; i++) {
12	        printf("%lu\n", i);
13	        sleep(1);
14	    }
15	    return 0;
16	}
```

We run the `samples/sleepy_print` program with `deet`, setting the initial breakpoint at line 12 (the `printf` call), and then set another breakpoint at line 13 (the `sleep` call) after the program is kick started. We continue the program until it exits, printing out the numbers as it goes.

```sh
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/deet samples/sleepy_print`
------
samples/sleepy_print.c
------
Global variables:
Functions:
  * sleep (declared on line 464, located at 0x0, 0 bytes long)
  * printf (declared on line 356, located at 0x0, 0 bytes long)
  * exit (declared on line 624, located at 0x0, 0 bytes long)
  * fprintf (declared on line 350, located at 0x0, 0 bytes long)
  * strtoul (declared on line 181, located at 0x0, 0 bytes long)
  * main (declared on line 5, located at 0x4011b6, 181 bytes long)
    * Variable: argc (int, located at FramePointerOffset(-36), declared at line 5)
    * Variable: num_seconds (long unsigned int, located at FramePointerOffset(-24), declared at line 6)
    * Variable: i (long unsigned int, located at FramePointerOffset(-32), declared at line 11)
Line numbers:
  * 5 (at 0x4011b6)
  * 7 (at 0x4011c9)
  * 7 (at 0x4011cf)
  * 7 (at 0x4011d7)
  * 7 (at 0x4011f0)
  * 8 (at 0x4011f7)
  * 9 (at 0x40121c)
  * 11 (at 0x401226)
  * 11 (at 0x40122e)
  * 12 (at 0x401230)
  * 13 (at 0x40124b)
  * 11 (at 0x401255)
  * 11 (at 0x40125a)
  * 15 (at 0x401264)
  * 16 (at 0x401269)
(deet) b *0x401230
Set breakpoint 0 at 0x401230
(deet) r
Usage: samples/sleepy_print <seconds to sleep>
Child exited (status 1)
(deet) r 5
Child stopped (signal SIGTRAP)
Stopped at ./samples/sleepy_print.c:12
(deet) cont
0
Child stopped (signal SIGTRAP)
Stopped at ./samples/sleepy_print.c:12
(deet) cont
1
Child stopped (signal SIGTRAP)
Stopped at ./samples/sleepy_print.c:12
(deet) b *0x40124b
Set breakpoint 1 at 0x40124b
(deet) cont
2
Child stopped (signal SIGTRAP)
Stopped at ./samples/sleepy_print.c:13
(deet) cont
Child stopped (signal SIGTRAP)
Stopped at ./samples/sleepy_print.c:12
(deet) cont
3
Child stopped (signal SIGTRAP)
Stopped at ./samples/sleepy_print.c:13
(deet) cont
Child stopped (signal SIGTRAP)
Stopped at ./samples/sleepy_print.c:12
(deet) cont
4
Child stopped (signal SIGTRAP)
Stopped at ./samples/sleepy_print.c:13
(deet) cont
Child exited (status 0)
```

Our debugger successfully set breakpoints before and after the execution of the program, resume execution, and print out the numbers as expected.
