// S3 Practice 05 — Functions (`fn`), Expressions vs. Statements (`E0308`), and Early `return`
// Copy-paste into https://play.rust-lang.org/ and click Run!

// School algebra: f(x) = 50x
// Rust function:  takes `cups: i32` and `price: i32` (parameters) -> returns `i32`
fn calc_total(cups: i32, price: i32) -> i32 {
    // Early return guard (no trailing `;` needed in single-expression block)
    if cups <= 0 {
        return 0
    }

    // Expression (NO semicolon) -> returns the calculated i32 value!
    // Try adding `;` at the end of the next line to trigger:
    //   error[E0308]: mismatched types — expected `i32`, found `()`
    cups * price
}

fn main() {
    // Passing `3` and `50` (arguments)
    let bill = calc_total(3, 50);
    println!("Total for 3 cups: {bill} EGP");

    let empty_order = calc_total(0, 50);
    println!("Total for 0 cups: {empty_order} EGP");
}
