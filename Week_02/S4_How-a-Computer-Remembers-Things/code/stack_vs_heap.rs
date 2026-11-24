// S4 — How a Computer Remembers Things (Stack vs. Heap)
// Run this file directly in Rust Playground: https://play.rust-lang.org/

fn add_tax(price: i32) -> i32 {
    let tax: i32 = 5;
    price + tax // Stack Frame (price, tax, return address) pops right here!
}

fn main() {
    // 1. Fixed-size values on the Stack + inner temporary scope block
    let cups: i32 = 3; // 4 bytes on the Stack
    let is_hot: bool = true; // 1 byte on the Stack
    {
        let temp: i32 = 42; // Lives only inside this { } scope
        println!("Inside scope -> cups: {cups}, is_hot: {is_hot}, temp: {temp}");
    } // `temp` pops off the Stack at this closing brace!

    // 2. Stack Frame push/pop with a function call (from S3)
    let total = add_tax(100);
    println!("Total returned from add_tax: {total}");

    // 3. Growable String on the Heap (24-byte ptr/len/cap header on the Stack)
    let mut order = String::from("Pavi");
    println!(
        "Before push_str -> order: \"{order}\", len: {}, cap: {}",
        order.len(),
        order.capacity()
    );

    order.push_str(" + Extra Shot");
    println!(
        "After  push_str -> order: \"{order}\", len: {}, cap: {}",
        order.len(),
        order.capacity()
    );

    // 4. Quick mental model: &str (fixed, no Heap) vs. String (growable, Heap)
    let label: &str = "Pavi";
    println!("Fixed &str label: \"{label}\" (0 Heap allocations)");

    // 5. Same rule for other types: Arrays & Tuples (Stack) vs. Vec (Heap)
    let arr: [i32; 3] = [1, 2, 3]; // Fixed size -> Stack
    let pair: (i32, bool) = (5, true); // Fixed size -> Stack
    let mut nums: Vec<i32> = vec![1, 2, 3]; // Growable -> Heap
    nums.push(4);

    println!("Stack Array: {arr:?}, Stack Tuple: {pair:?}, Heap Vec: {nums:?}");
}
