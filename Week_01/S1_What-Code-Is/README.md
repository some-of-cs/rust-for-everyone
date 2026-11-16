# S1 — What Coding Means, Compilers & Your First Program

> **Outcome:** Explain the path from `Problem → Source Code → Build (Compiler) → Run (Output)`, distinguish natural-language AI assistance from formal programming languages, and verify a real calculation against its intended goal.

---

## 🧠 Core Concepts Covered
1. **Data, Algorithm & Output:** Just like a recipe has ingredients (`Data`), ordered preparation steps (`Algorithm`), and a final dish (`Output`), a program takes data and executes ordered instructions to produce an observable result.
2. **Natural Language & AI Assistance vs. Source Code:** In traditional programming, you supply the `Input` and write the `Algorithm` to get the `Output`. With AI models (LLMs), you describe the `Input` and expected `Output` in natural language and the model proposes an `Algorithm`—which you must still inspect and verify yourself.
3. **Abstraction Levels:** From `Low-Level` (Machine Code `0`s and `1`s) → `Middle-Level` (Assembly / C direct memory manipulation) → `High-Level` (human-readable syntax like Python, JavaScript, C++, and Rust) → `Natural-Level` prompts. Rust combines high-level readability and safety with low-level hardware control.
4. **Compiler vs. Interpreter & Logic Checks:** The Rust compiler translates your `Source Code` into a standalone `Executable` during the `Build` step. However, **a successful build only proves your code follows language rules—not that your business logic is correct** (e.g., adding instead of subtracting compiles cleanly but produces the wrong customer change!).

---

## 💻 Code Practice (Run in [Rust Playground](https://play.rust-lang.org/))

### 1. First Run & Cafe Bill Calculation
Copy and run [`code/cafe_calculator.rs`](./code/cafe_calculator.rs):

```rust
fn main() {
    // 1. First Greeting
    println!("Hello, Rust for Everyone!");

    // 2. Cafe Order Calculation:
    // Coffee price = 40 EGP, Quantity = 3, Customer paid = 200 EGP
    println!("Total bill: {}", 40 * 3);
    println!("Customer change: {}", 200 - 40 * 3);
}
```

### 2. Valid Code vs. Intended Behavior (Logic Bug Experiment)
What happens if we accidentally write `+` instead of `-` when calculating the customer's change?
```rust
fn main() {
    println!("Total bill: {}", 40 * 3);
    // Compiles without errors, but prints 320 instead of 80!
    println!("Customer change: {}", 200 + 40 * 3);
}
```

### 3. Compiler Error Experiment
Remove the `!` from `println!` and read the compiler's diagnostic message before fixing it:
```rust
fn main() {
    // println("Total bill: {}", 40 * 3); // Compiler catches missing `!` before the program runs
    println!("Total bill: {}", 40 * 3);
}
```

---

## 🎯 Your Action (CTA)
Change the coffee quantity from `3` to `4` in the code above, **write down your prediction for the Total Bill and Customer Change before clicking `Run`**, and then run the program to verify!
