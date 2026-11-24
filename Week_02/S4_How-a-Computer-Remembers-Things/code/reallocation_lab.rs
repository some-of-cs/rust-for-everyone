// S4 Lab — Watch Heap Reallocation Happen Live (Locker #402 -> Locker #815)
// Run in Rust Playground: https://play.rust-lang.org/

use std::mem::size_of;

fn main() {
    // Part 1: Compile-time known Stack sizes
    println!("=== Fixed Stack Sizes ===");
    println!("i32    -> {} bytes on Stack", size_of::<i32>());
    println!("bool   -> {} byte  on Stack", size_of::<bool>());
    println!(
        "String -> {} bytes on Stack (ptr + len + cap)",
        size_of::<String>()
    );
    println!();

    // Part 2: Observe the Heap pointer address BEFORE and AFTER growing past capacity
    let mut order = String::from("Pavi"); // 4 bytes: len = 4, cap = 4
    let ptr_before = order.as_ptr();
    println!("=== Before .push_str ===");
    println!(
        "text: \"{order}\" | heap ptr: {:p} | len: {} | cap: {}",
        ptr_before,
        order.len(),
        order.capacity()
    );

    // Exceed capacity -> Allocator reserves a bigger Heap block, copies bytes, frees old block
    order.push_str(" + Extra Shot"); // +13 bytes -> total len = 17
    let ptr_after = order.as_ptr();
    println!("\n=== After .push_str ===");
    println!(
        "text: \"{order}\" | heap ptr: {:p} | len: {} | cap: {}",
        ptr_after,
        order.len(),
        order.capacity()
    );

    println!(
        "\nDid the Heap pointer move to a new locker? {}",
        ptr_before != ptr_after
    );
    println!(
        "String header size on Stack is STILL {} bytes!",
        size_of::<String>()
    );
}
