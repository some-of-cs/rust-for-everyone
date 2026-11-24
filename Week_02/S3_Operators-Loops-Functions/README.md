# S3 — Operators, Decisions (`if / else if`), Loops & Functions

> **Outcome:** Use Rust's arithmetic, comparison, compound assignment, logical, and bitwise operators, write bug-free `if / else if / else` chains without ordering traps, choose cleanly between `loop`, `while`, and `for` (including `break value`, `continue`, `1..3` vs `1..=3`, and `.rev()`), and build reusable functions while distinguishing **Expressions** from **Statements** (`E0308`).

---

## 🧠 Core Concepts Covered

1. **Operators in Rust (Arithmetic, Comparison, Compound, Logical & Bitwise):**
   - **Arithmetic:** `+`, `-`, `*`, `/`, and `%` (remainder, e.g., `10 % 3 == 1`).
   - **Comparison:** `==`, `!=`, `>`, `<`, `>=`, `<=` (always return a `bool`).
   - **Compound Assignment:** `+=`, `-=`, `*=`, `/=`, `%=` require `let mut`. Rust has **no `++` or `--`** operators — always write `+= 1` or `-= 1`.
   - **Logical (`bool`):** `&&` (AND), `||` (OR), `!` (NOT).
   - **What Is a Bit & Bitwise Operators:** Each bit is a switch (`0` = OFF, `1` = ON) weighted by powers of 2 from right to left (`8 | 4 | 2 | 1`), so `6` is `0b110` and `3` is `0b011`. Bitwise operators work bit-by-bit: `&` (AND), `|` (OR), `^` (XOR), `!` (NOT — Rust uses `!`, not `~`), `<<` (Left Shift — pushes bits left and fills zeros on the right, equivalent to multiplying by `2^n`), and `>>` (Right Shift — pushes bits right, equivalent to dividing by `2^n`). All have compound forms (`&=`, `|=`, `^=`, `<<=`, `>>=`).
   - **Production vs. LeetCode Reality Check:** In day-to-day production code, you will mostly use arithmetic and logical operators and rarely touch bitwise operators — you will still encounter bitwise tricks in LeetCode and coding interviews.

2. **Deeper into `if / else` & `if / else if / else`:**
   - `if / else` is an **Expression** in Rust: it can return a value directly into a `let` binding, provided every branch returns the exact same type and omits the trailing `;`.
   - `if / else if / else` chains evaluate **top-down** and stop at the first branch that evaluates to `true`. Always place the **most specific / highest threshold first** (`cups >= 5` before `cups >= 3`), otherwise `cups = 6` gets caught by `>= 3` and never reaches `>= 5`.

3. **Loops (`loop`, `while`, `for`):**
   - **`loop`:** Runs forever until `break`. Always update your state (e.g., `orders += 1;`) to avoid an infinite loop. Uniquely in Rust, `loop` can return a value via `break value;`. Use `continue;` to skip the rest of the current iteration and jump back to the top.
   - **`while`:** Repeats while a condition stays `true` (`while queue > 0`). If the condition starts `false`, the body runs zero times.
   - **`for`:** The safest loop in Rust — no manual counter to forget. Watch range boundaries: `1..3` is **exclusive** (`1, 2`), while `1..=3` is **inclusive** (`1, 2, 3`). Wrap a range in parentheses and call `.rev()` to count down (`(1..=3).rev()`).
   - **Analogy Boundary:** Unlike `loop`, `while` and `for` cannot return a value with `break value`, because the compiler cannot guarantee they will run even once.

4. **Functions (`fn`), Parameters vs. Arguments, and Expressions vs. Statements:**
   - Think of school algebra: `f(x) = 50x` maps 1-to-1 to `fn calc_total(cups: i32) -> i32 { cups * 50 }`.
   - **Parameter** = the named typed slot in the function signature (`cups: i32`); **Argument** = the concrete value passed at the call site (`3`).
   - **Statement vs. Expression (The Opening `E0308` Mystery):**
     - A **Statement** performs an action and returns `()` (the Unit Type), e.g., `let cups = 3;` or `cups * 50;`.
     - An **Expression** evaluates to a value, e.g., `cups * 50` (no trailing `;`).
     - Adding a `;` to the final line of a function that promises `-> i32` turns it into a statement returning `()`, triggering `error[E0308]: expected i32, found ()`.
   - Use `return` only for **early exits** (e.g., `if cups <= 0 { return 0 }`); leave the final return line as an expression without `return` or `;`.

---

## 💻 Practice Files (Run in [Rust Playground](https://play.rust-lang.org/))

Every file under [`code/`](./code) is self-contained and ready to copy-paste straight into [Rust Playground](https://play.rust-lang.org/) (scoped strictly to `i32`, `&str`, `bool`, and `()`):

| # | File | What You Will Practice |
| :--- | :--- | :--- |
| 1 | [`code/01_operators_and_bitwise.rs`](./code/01_operators_and_bitwise.rs) | Arithmetic (`+ - * / %`), compound assignment (`+= -= *= /= %=`), logical (`&& \|\| !`), binary literals (`0b110`, `0b011`), bitwise (`& \| ^ !`), and bit shifts (`<<`, `>>`). |
| 2 | [`code/02_if_else_and_discounts.rs`](./code/02_if_else_and_discounts.rs) | `if / else` as an expression returning `&str`, plus `if / else if / else` cafe discount tiers and the top-down ordering trap (`cups = 6`). |
| 3 | [`code/03_loops_break_continue.rs`](./code/03_loops_break_continue.rs) | `loop` with `break`, infinite loop trap, returning a value via `break orders * 50;`, and skipping a canceled order with `continue;`. |
| 4 | [`code/04_while_and_for_ranges.rs`](./code/04_while_and_for_ranges.rs) | `while queue > 0` customer line, `for cup in 1..3` vs `1..=3` boundary check, and `(1..=3).rev()` countdown timer. |
| 5 | [`code/05_functions_and_expressions.rs`](./code/05_functions_and_expressions.rs) | School algebra `f(x) = 50x` in Rust, Expressions vs. Statements (`E0308` semicolon trap), and early `return 0`. |
| 6 | [`code/06_cafe_cashier.rs`](./code/06_cafe_cashier.rs) | Full Episode #04 Cafe Cashier combining `calc_discount`, `calc_total` (with early `return 0`), and a `for cups in 1..=5` loop. |

### Featured Snippet — Complete Cafe Cashier ([`code/06_cafe_cashier.rs`](./code/06_cafe_cashier.rs))
```rust
fn calc_discount(cups: i32) -> i32 {
    if cups >= 5 {
        20
    } else if cups >= 3 {
        10
    } else {
        0
    }
}

fn calc_total(cups: i32, price: i32) -> i32 {
    if cups <= 0 {
        return 0
    }
    let gross = cups * price;
    let discount = calc_discount(cups);
    gross - (gross * discount / 100)
}

fn main() {
    println!("Zero-cup check: {} EGP", calc_total(0, 50));

    for cups in 1..=5 {
        let bill = calc_total(cups, 50);
        println!("{cups} cups -> {bill} EGP");
    }
}
```

---

## 📝 4-Question Mini-Quiz

Predict all four answers before checking!

1. If you write `if cups >= 3 { 10 } else if cups >= 5 { 20 } else { 0 }` and pass `cups = 6`, what discount is returned?
2. What is the last number printed by `for n in 1..4 { println!("{n}"); }`?
3. Does `fn double(x: i32) -> i32 { x * 2; }` compile? Why or why not?
4. Can you return a value from a `while` loop using `break 10;`?

<details>
<summary>Show Answers</summary>

1. **`10` (10%)**, not 20% — Rust checks branches top-down and stops at the first `true` condition (`6 >= 3`).
2. **`3`** — `1..4` is exclusive of `4`. Use `1..=4` to include `4`.
3. **No** — it fails with `error[E0308]: expected i32, found ()` because the trailing `;` turns `x * 2` into a statement returning `()`.
4. **No** — `break value` works only with `loop`, because a `while` loop might run zero times.

</details>

---

## 🎯 Your Action (CTA)
Open [`code/06_cafe_cashier.rs`](./code/06_cafe_cashier.rs) in [Rust Playground](https://play.rust-lang.org/), try intentionally breaking one rule at a time (swap the `>= 5` and `>= 3` branches, change `1..=5` to `1..5`, or add a `;` to the last line of `calc_total`), **predict what will happen before pressing `Run`**, and share your Mini-Quiz score out of 4 in [GitHub Discussions](https://github.com/some-of-cs/rust-for-everyone/discussions)!
