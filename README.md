# Safety in Systems Programming

The Repository contains the learning materials of the [Safety in Systems Programming](https://reberhardt.com/cs110l/spring-2020/) course by Ryan Eberhardt and Armin Namavari.

This course is focused on safety and robustness in systems programming: 

- Where do things often go wrong in computer systems? 
- How can we avoid common pitfalls? 
- Use the Rust programming language as a vehicle to teach **mental models and paradigms** that have been shown to be helpful in preventing errors
- How these features have made their way back into C++.

---

# Projects

## 1. Simple Debugger

This project will give us practice with multiprocessing in Rust, and will give us a better sense of how processes are managed by the operating system as well as how `ptrace` can be used to circumvent process boundaries.

Features:

1. run, stop, continue, kill the traced process
1. print a stack trace for a paused program
1. set the breakpoints

Implementation: [Here](proj-1/deet)

## 2. Load Balancer

This project will give us practice with multithreading, asynchronous programming, and performance optimization by building a load balancer using [tokio](https://docs.rs/tokio/1.0.0/tokio/index.html) runtime.

Features:

1. Random load balancing

Implementation: [Work In Progress](proj-2/balancebeam)

---

# Programming Assigments

1. [Hangman Game](week1)
1. [File Diff Tool](week2/rdiff)
1. [File Descriptor Inspector](week3/inspect-fds)
1. [Generic Linked List](week3/linked_list)
1. [Farm meets multithreading](week5/farm)
1. [Parallel Map](week6)
