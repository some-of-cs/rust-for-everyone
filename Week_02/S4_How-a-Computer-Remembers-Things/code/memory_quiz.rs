// S4 Mini-Quiz — Stack or Heap?
// Predict your answers BEFORE clicking Run in Rust Playground: https://play.rust-lang.org/

fn add_tax(price: i32) -> i32 {
    let tax: i32 = 5;
    price + tax
}

fn main() {
    // Question 1: Where does `is_ready` live?
    let is_ready: bool = true;
    println!("Q1 (`is_ready = {is_ready}`): STACK — `bool` is fixed at 1 byte (Compile Time).");

    // Question 2 (Trick!): Where does `greeting` live?
    let mut greeting = String::from("Hello");
    greeting.push_str("!");
    println!(
        "Q2 (`greeting = \"{greeting}\"`): BOTH — (ptr, len, cap) header on STACK, UTF-8 bytes on HEAP."
    );

    // Question 3: Where does `tax` inside `add_tax(100)` live, and when does it disappear?
    let total = add_tax(100);
    println!(
        "Q3 (`add_tax(100) = {total}`): STACK FRAME — pushed when `add_tax` starts, popped the instant it returns."
    );

    // Bonus Question 4: Fixed Array & Tuple vs. Growable Vec
    let scores: [i32; 3] = [10, 20, 30];
    let status: (i32, bool) = (200, true);
    let mut cart: Vec<i32> = vec![1, 2, 3];
    cart.push(4);
    println!(
        "Bonus (`scores = {scores:?}`, `status = {status:?}`): STACK (fixed size)."
    );
    println!(
        "Bonus (`cart = {cart:?}`): Header on STACK, growable elements on HEAP."
    );
}
