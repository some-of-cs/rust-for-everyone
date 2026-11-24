// S3 Practice 03 — `loop`, `break`, Returning a Value from `loop`, and `continue`
// Copy-paste into https://play.rust-lang.org/ and click Run!

fn main() {
    // 1. Basic `loop` with `break`
    // Try deleting `orders += 1;` to see an infinite loop (Playground will time out).
    let mut orders = 0;
    loop {
        orders += 1;
        println!("Order #{orders} ready");
        if orders == 3 {
            break;
        }
    }

    // 2. Returning a value from `loop` via `break value;`
    let mut completed = 0;
    let daily_revenue = loop {
        completed += 1;
        if completed == 3 {
            break completed * 50;
        }
    };
    println!("Revenue: {daily_revenue} EGP");

    // 3. Skipping one canceled order with `continue;`
    // Notice `order += 1;` is at the TOP so `continue;` never skips the counter update.
    let mut order = 0;
    loop {
        order += 1;
        if order == 2 {
            println!("Order #2 canceled — skipping!");
            continue;
        }
        println!("Making coffee for Order #{order}");
        if order == 4 {
            break;
        }
    }
}
