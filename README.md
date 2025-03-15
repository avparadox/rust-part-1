# Rust Programming 🚀

This repository contains my practice code for learning Rust programming. It covers various concepts such as structs, variables, loops, conditionals, pattern matching, ownership, references, and more. Each section includes an explanation and example code to reinforce learning.

📄 **Docs File:** [Rust Bootcamp](https://projects.100xdevs.com/tracks/rust-bootcamp/Rust-Bootcamp-1)

---

## 📖 Table of Contents

1. [📝 Basic Input/Output](#basic-inputoutput)
2. [🏗️ Structs](#structs)
3. [🔢 Signed & Unsigned Integers](#signed--unsigned-integers)
4. [🔄 Mutable Variables & Loops](#mutable-variables--loops)
5. [✅ Booleans](#booleans)
6. [📜 Strings, Pattern Matching, and Option](#strings-pattern-matching-and-option)
7. [🔀 Conditionals](#conditionals)
8. [🔁 Loops](#loops)
9. [📌 Functions](#functions)
10. [🧠 Stack & Heap Memory](#stack--heap-memory)
11. [🛠️ Ownership & References](#ownership--references)
12. [⚠️ Error Handling](#error-handling)
13. [🎭 Enums](#enums)
14. [📏 Lifetimes](#lifetimes)
15. [📐 Traits](#traits)
16. [⚡ Concurrency](#concurrency)

---

## 📝 Basic Input/Output

<details>
<summary>Click to expand</summary>

### 🧐 Explanation

Rust's standard input/output operations use the `std::io` module. This example takes two numbers as input and prints their sum.

#### 📌 Additional Concepts:

- `read_line` method for user input
- `trim()` to remove newline characters
- `parse()` to convert strings to numbers
- Handling errors using `expect()`

```rust
use std::io;

fn main() {
    println!("Enter two numbers:");
    let mut input1 = String::new();
    let mut input2 = String::new();

    io::stdin().read_line(&mut input1).expect("Failed to read input");
    io::stdin().read_line(&mut input2).expect("Failed to read input");

    let num1: f64 = input1.trim().parse().expect("Invalid number");
    let num2: f64 = input2.trim().parse().expect("Invalid number");

    println!("The sum is: {}", num1 + num2);
}
```

</details>

---

## 🏗️ Structs

<details>
<summary>Click to expand</summary>

### 🧐 Explanation

Structs allow defining custom data types that group multiple related fields together.

#### 📌 Additional Concepts:

- Struct field access using `.` notation
- Implementing methods using `impl`
- Structs can have methods for functionality

```rust
struct User {
    name: String,
    age: u32,
    email: String,
    active: bool,
}

impl User {
    fn is_active(&self) -> bool {
        self.active
    }
}

fn main() {
    let user = User {
        name: String::from("Aditya"),
        age: 22,
        email: String::from("aditya@av.com"),
        active: true,
    };

    println!("User is active: {}", user.is_active());
}
```

</details>

---

## 📌 Functions

<details>
<summary>Click to expand</summary>

### 🧐 Explanation

Functions in Rust are used to organize and reuse code efficiently.

#### 📌 Additional Concepts:

- Function parameters and return types
- Using `->` to specify return type
- Returning early with `return`

```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    let sum = add(5, 7);
    println!("The sum is: {}", sum);
}
```

</details>

---

## 🛠️ Ownership & References

<details>
<summary>Click to expand</summary>

### 🧐 Explanation

Ownership ensures memory safety by enforcing unique data ownership.

#### 📌 Additional Concepts:

- Move semantics
- Borrowing with `&` references
- Mutable references with `&mut`

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1: {}", s1);
    println!("s2: {}", s2);
}
```

</details>

---

## ⚠️ Error Handling

<details>
<summary>Click to expand</summary>

### 🧐 Explanation

Rust provides error handling using `Result` and `Option` enums.

#### 📌 Additional Concepts:

- Unwrapping safely with `match`
- Propagating errors using `?`
- Handling `None` cases in `Option`

```rust
fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        return Err(String::from("Cannot divide by zero"));
    }
    Ok(numerator / denominator)
}

fn main() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
```

</details>

---

## ⚡ Concurrency

<details>
<summary>Click to expand</summary>

### 🧐 Explanation

Rust provides concurrency through threads and message passing.

#### 📌 Additional Concepts:

- Creating threads with `std::thread`
- Using `join()` to wait for threads
- Message passing with `mpsc`

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Hello from thread: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    handle.join().unwrap();
}
```

</details>

---

Happy Coding!
