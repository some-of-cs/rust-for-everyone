# S2 — Variables, Types, Shadowing & Decisions (`if / else`)

> **Outcome:** Bind names to values with `let`, understand `Type Inference`, distinguish source-code edits from in-run mutation (`let mut`), use `const` and `Shadowing` (including mutating a variable and freezing it again), and predict `if / else` branches including exact boundary values.

---

## 🧠 Core Concepts Covered
1. **Bindings & Type Inference (`let`):** Connecting a name (`name`) to a value (`"Pavi"`) is called a **binding** in Rust. Rust automatically infers the data type from the value (`Type Inference`), and the English variable name never overrides the actual value's type.
2. **Immutability by Default, `let mut` & `const`:**
   - Editing code and clicking `Run` again is a brand-new program execution.
   - Changing a variable's value *within the same run* is prevented by default (`E0384`) unless you explicitly mark it with `let mut`.
   - `const` defines a compile-time constant that never accepts `mut`, requires an explicit type annotation (e.g., `const COFFEE_PRICE: i32 = 40;`), and uses `UPPERCASE` naming.
3. **Shadowing (Mutate Then Freeze & Type Transformation):**
   - Re-declaring a variable with `let` creates a new binding with the same name.
   - You can start with `let mut price = 100;`, modify it, and then write `let price = price;` to **freeze** it as immutable again!
   - You can also transform a value's type while keeping the same meaningful name (e.g., `let spaces = "   "; let spaces = spaces.len();`).
4. **Booleans & `if / else` Boundary Decisions:**
   - Comparisons like `paid >= 120` evaluate to an explicit `bool` (`true` or `false`). Rust never treats raw numbers as booleans.
   - Always inspect the exact operator (`>=` vs `>`) on boundary values (`119`, `120`, `121`).

---

## 💻 Code Practice (Run in [Rust Playground](https://play.rust-lang.org/))

See [`code/cafe_decisions.rs`](./code/cafe_decisions.rs) for the complete runnable examples:

### 1. `const`, `let`, `let mut` & Cafe Total
```rust
const COFFEE_PRICE: i32 = 40;

fn main() {
    let quantity = 3;
    let total = COFFEE_PRICE * quantity;

    let paid = 200;
    let mut tip = 0;
    tip = 10;

    println!("Total: {total}, Paid: {paid}, Tip: {tip}");
}
```

### 2. Shadowing: Mutate a Variable, Then Freeze It Again
```rust
fn main() {
    let mut price = 100;
    price += 20; // Allowed while mutable

    let price = price; // Frozen again via Shadowing!
    // price = 150; // Uncommenting this triggers compiler error E0384

    let spaces = "   ";
    let spaces = spaces.len(); // Shadowing with a new type (usize)

    println!("Locked price: {price}, Spaces: {spaces}");
}
```

### 3. `if / else` & Boundary Testing (`119`, `120`, `121`)
```rust
fn main() {
    let paid = 120; // Test with: 200, 100, 120, 119, 121

    if paid >= 120 {
        println!("Payment accepted! Here is your coffee.");
    } else {
        println!("Insufficient amount. Total is 120.");
    }
}
```

---

## 🎯 Your Action (CTA)
Open the decision code in [Rust Playground](https://play.rust-lang.org/), test `paid` with a number below `120` (like `119`) and at `120` or above, **write down your prediction before clicking `Run`**, and compare your prediction with the result!
