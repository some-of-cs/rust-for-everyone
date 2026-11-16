const COFFEE_PRICE: i32 = 40;

fn main() {
    // 1. Binding & Type Inference
    let name = "Pavi";
    println!("Hello, {name}!");

    // 2. Constants, Immutability, Mutability & Cafe Total
    let quantity = 3;
    let total = COFFEE_PRICE * quantity;

    let paid = 200;
    let mut tip = 0;
    tip = 10;

    // 3. Shadowing: Freeze `tip` so it can no longer be mutated
    let tip = tip;

    println!("Total: {total}, Paid: {paid}, Tip: {tip}");

    // 4. Decisions & Boundary Check (Try paid = 200, 100, 120, 119, 121)
    if paid >= total {
        println!("Payment accepted! Here is your coffee.");
    } else {
        println!("Insufficient amount. Total is {total}.");
    }
}
