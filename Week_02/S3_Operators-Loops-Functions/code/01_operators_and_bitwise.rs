// S3 Practice 01 — Arithmetic, Compound Assignment, Logical & Bitwise Operators
// Copy-paste into https://play.rust-lang.org/ and click Run!

fn main() {
    // 1. Arithmetic & Remainder (%)
    let remainder = 10 % 3;
    println!("10 % 3 = {remainder}");

    // 2. Compound Assignment (requires `let mut`; Rust has NO ++ or --)
    let mut cups = 2;
    cups += 3; // 5
    cups -= 1; // 4
    cups *= 2; // 8
    println!("Cups now: {cups}");

    // 3. Logical Operators (&&, ||, !) on bool
    let is_open = true;
    let paid = 120;
    let total = 100;
    let can_serve = is_open && paid >= total;
    println!("Can serve: {can_serve}");

    let is_vip = false;
    let gets_priority = is_vip || cups >= 5;
    println!("Priority order: {gets_priority}");
    println!("Shop closed? {}", !is_open);

    // 4. Bits & Bitwise Operators (&, |, ^, !, <<, >>)
    // Powers of 2 from right to left: 4 | 2 | 1
    // 6 = 4 + 2 -> 0b110
    // 3 = 2 + 1 -> 0b011
    let a = 6; // 0b110
    let b = 3; // 0b011
    println!("AND: {}, OR: {}, XOR: {}", a & b, a | b, a ^ b);

    // Bit Shifts:
    // Left shift (<<) pushes bits left and fills zeros on the right (x << n == x * 2^n)
    // Right shift (>>) pushes bits right and drops rightmost bits   (x >> n == x / 2^n)
    println!("3 << 2 = {}", 3 << 2);   // 0b0011 -> 0b1100 (12)
    println!("12 >> 2 = {}", 12 >> 2); // 0b1100 -> 0b0011 (3)

    // Note: In production apps you will mostly use arithmetic and logical operators.
    // Bitwise operators show up less often in business logic, but frequently in LeetCode and interviews.
}
