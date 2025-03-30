### Day 2: Ownership & Borrowing in Rust

    Rust’s memory management is unique because it doesn’t rely on a garbage collector. Instead, it uses ownership, borrowing, and lifetimes to ensure memory safety at compile time. Understanding these concepts is crucial to writing efficient and safe Rust programs.

1. Ownership: The Core of Rust’s Memory Model
   Definition: Ownership is a set of rules that govern how Rust manages memory. Every value in Rust has a single owner, and when the owner goes out of scope, the value is automatically dropped (freed from memory).

✅ Rules of Ownership: - Each value has a single owner. - When the owner goes out of scope, the value is dropped (memory is freed). - You cannot transfer ownership without explicitly doing so.

```
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // Ownership of s1 is moved to s2

    // println!("{}", s1); // ❌ ERROR! s1 is no longer valid
    println!("{}", s2); // ✅ Works fine
}

```

cloning to Copy Data
If you want to keep using s1, use .clone():

```
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone(); // Creates a deep copy

    println!("{}", s1); // ✅ Works fine
    println!("{}", s2);
}

```

2. Borrowing & References: Sharing Data Without Ownership Transfer
   Ownership is strict, but Rust allows borrowing, meaning we can temporarily lend ownership without transferring it.

✅ Borrowing Rules:

    1. At any time, only one mutable reference or multiple immutable references can exist.
    2. References cannot outlive the data they point to.

    Immutable Borrowing (Read-Only Access)

```
fn print_length(s: &String) { // `s` is a reference, no ownership change
    println!("Length: {}", s.len());
}

fn main() {
    let s = String::from("Hello");
    print_length(&s); // Borrowing `s`
    println!("{}", s); // ✅ Still valid
}

```

Mutable Borrowing (Write Access)

```
fn change(s: &mut String) {
    s.push_str(", world!");
}

fn main() {
    let mut s = String::from("Hello");
    change(&mut s); // Mutable borrow
    println!("{}", s); // ✅ "Hello, world!"
}

```

👉 You can only have one mutable reference at a time to prevent data races.

### Invalid Borrowing: Mutable + Immutable Conflict

```
fn main() {
    let mut s = String::from("Hello");

    let r1 = &s; // Immutable borrow
    let r2 = &s; // Immutable borrow
    let r3 = &mut s; // ❌ ERROR! Cannot borrow as mutable while immutable references exist

    println!("{}, {}", r1, r2);
    //👉 You cannot mix mutable and immutable references simultaneously.


}


```

3. Lifetimes: Preventing Dangling References
   Lifetimes ensure that references do not outlive the data they refer to.

Dangling Reference Example (Invalid)

```

fn dangle() -> &String { // ❌ ERROR! Returns reference to a value that will be dropped
    let s = String::from("Hello");
    &s // ❌ s is destroyed after function exits
}

fn main() {
    let reference = dangle();
}

//👉 The returned reference would point to invalid memory, so Rust doesn’t ////allow this.



```

Corrected with Lifetimes

```
fn no_dangle<'a>(s: &'a String) -> &'a String { // Lifetime 'a ensures `s` outlives the function
    s
}

fn main() {
    let s = String::from("Hello");
    let reference = no_dangle(&s);
    println!("{}", reference); // ✅ Safe
}
//👉 'a tells the compiler that the reference must be valid as long as s is valid.
```
