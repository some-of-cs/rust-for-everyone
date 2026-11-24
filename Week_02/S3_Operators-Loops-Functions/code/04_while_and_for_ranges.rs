// S3 Practice 04 — `while` Queue, `for` Ranges (`1..3` vs `1..=3`), and `.rev()`
// Copy-paste into https://play.rust-lang.org/ and click Run!

fn main() {
    // 1. `while` loop — condition lives in the header
    let mut queue = 3;
    while queue > 0 {
        println!("Serving customer, {queue} left");
        queue -= 1;
    }
    println!("Line is empty!");

    // 2. `for` with Exclusive Range (`1..3` prints 1 and 2, excludes 3)
    println!("--- Exclusive 1..3 ---");
    for cup in 1..3 {
        println!("Pouring espresso for cup #{cup}");
    }

    // 3. `for` with Inclusive Range (`1..=3` prints 1, 2, and 3)
    println!("--- Inclusive 1..=3 ---");
    for cup in 1..=3 {
        println!("Pouring espresso for cup #{cup}");
    }

    // 4. Countdown using `.rev()`
    println!("--- Espresso Timer ---");
    for sec in (1..=3).rev() {
        println!("Espresso ready in {sec}...");
    }
    println!("Serve!");
}
