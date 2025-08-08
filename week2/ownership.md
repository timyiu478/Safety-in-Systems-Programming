Example 1:

```rust
fn main() {
    let mut s = String::from("hello");
    let ref1 = &s;
    let ref2 = &ref1;
    let ref3 = &ref2;
    s = String::from("goodbye"); // s ownership is borrowed
    println!("{}", ref3.to_uppercase()); // to_uppercase() creates a new string "HELLO" instead of modifying "hello" to "HELLO"
}
```

This does not compiled.

---

Example 2:

```rust
fn drip_drop() -> &String {
    let s = String::from("hello world!");
    return &s; // s is local variable of drip_drop, once this function returns, the s variable will be dropped.
}
```

This does not compiled.

---

Example 3:

```rust
fn main() {
    let s1 = String::from("hello");
    let mut v = Vec::new();
    v.push(s1);
    let s2: String = v[0]; // s2 cant reference to mutable data
    println!("{}", s2);
}
```

This does not compiled.
