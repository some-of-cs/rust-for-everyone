// S3 Practice 02 — `if / else` Expressions & `if / else if / else` Ordering Trap
// Copy-paste into https://play.rust-lang.org/ and click Run!

fn main() {
    // 1. `if / else` as an Expression returning `&str`
    // Notice: no semicolon after each branch string, and `;` closes `let status = ...;`
    let total = 100;
    let paid = 120;

    let status = if paid >= total {
        "Hand over the coffee"
    } else {
        "Ask for the rest"
    };
    println!("Payment status: {status}");

    // 2. `if / else if / else` Cafe Discount Tiers
    // Always put the most specific / highest threshold FIRST (`>= 5` before `>= 3`).
    let cups = 6;
    let discount = if cups >= 5 {
        20
    } else if cups >= 3 {
        10
    } else {
        0
    };
    println!("Discount for {cups} cups: {discount}%");

    // 3. Ordering Trap Experiment:
    // With `cups = 6`, Rust stops at `cups >= 3` and returns 10% instead of 20%!
    let buggy_discount = if cups >= 3 {
        10
    } else if cups >= 5 {
        20
    } else {
        0
    };
    println!("Buggy order discount for {cups} cups: {buggy_discount}%");
}
