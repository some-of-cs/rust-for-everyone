// S3 Practice 06 — Full Cafe Cashier Integration
// Combines operators, `if / else if / else`, `for` inclusive range, early `return`, and expressions.
// Copy-paste into https://play.rust-lang.org/ and click Run!

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
