# Farm

This program will receive the numbers to factor using command line arguments. For each number, it determines the prime factors of a number and prints them to stdout.

# Example Usage

```
❯ cargo run 12345678 12346789 34567890 45678902 43853485 59854
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/farm 12345678 12346789 34567890 45678902 43853485 59854`
Farm starting on 8 CPUs
59854 = 2 * 29927 [time: 2.005819ms]
12345678 = 2 * 3 * 3 * 47 * 14593 [time: 385.592593ms]
12346789 = 7 * 13 * 19 * 37 * 193 [time: 393.516306ms]
34567890 = 2 * 3 * 5 * 7 * 97 * 1697 [time: 825.098219ms]
43853485 = 5 * 13 * 674669 [time: 982.59294ms]
45678902 = 2 * 433 * 52747 [time: 1.022889523s]
Total execution time: 1.023171887s
```
