# Simple Debugger

This project will give us practice with multiprocessing in Rust, and will give us a better sense of how processes are managed by the operating system as well as how `ptrace` can be used to circumvent process boundaries.

## Features

1. run, stop, continue, kill the traced process
1. print a stack trace for a paused program

## Commands

1. `r/run`: start the traced program, kill any existing traced program first
1. `c/cont`: continue the traced program if it is stopped 
1. `q/quit`: exit the debugger, killing any traced program first
1. `Ctrl-C`: stop the traced program
1. `bt/back/backtrace`: print a stack trace for the traced program if it is stopped
