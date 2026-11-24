# S4 — How a Computer Remembers Things (Stack vs. Heap)

> **Outcome:** Build an accurate behind-the-scenes mental model of how a computer stores fixed-size values on the **Stack** versus growable values on the **Heap**, explain why the Stack is faster even though both live in the same RAM, compare `&str` vs. `String` and `Array`/`Tuple` vs. `Vec`, and understand the **Double Free** problem that leads directly into **Ownership**.

---

## 🧠 Core Concepts Covered

1. **The Stack (Contiguous RAM + CPU Stack Pointer + Stack Frames):**
   - Stores values with a **fixed size known at Compile Time** (e.g., `i32` = `4 bytes`, `bool` = `1 byte`, fixed-size `Arrays`, and `Tuples`).
   - Operates in **LIFO** (*Last In, First Out*) order.
   - Allocating or freeing a variable moves a single CPU register — the **Stack Pointer** — in one instruction with zero searching.
   - Every function call (like `add_tax(100)`) pushes a **Stack Frame** holding its parameters, local variables, and **Return Address**, and pops the entire frame the instant the closing brace `}` hits (without zeroing out the old bytes).

2. **The Heap (Memory Allocator + `ptr`, `len`, `cap` + Reallocation):**
   - Stores values whose size can **grow or shrink at Runtime** (e.g., `String`, `Vec<T>`).
   - A `String` is split across **both** regions:
     - **On the Stack (`24 bytes` on 64-bit):** a fixed-size header holding `ptr` (memory address on the Heap), `len` (current bytes used), and `cap` (allocated byte capacity).
     - **On the Heap:** the actual UTF-8 bytes (e.g., `['P', 'a', 'v', 'i']`).
   - When `.push_str(...)` exceeds `cap`, the **Memory Allocator** performs a 4-step **Reallocation**: reserve a larger Heap block, copy the existing bytes over + append the new bytes, free the old block, and update `ptr` and `cap` on the Stack.

3. **Why Is the Stack Faster Than the Heap When Both Live in RAM?**
   - **Allocation Cost:** Moving one Stack Pointer register vs. searching the Allocator's free-list ledger.
   - **Direct Read vs. Pointer Indirection:** Reading a Stack value takes 1 step; reading a Heap value reads the Stack `ptr` first and then jumps to another RAM address.
   - **CPU Cache Locality (`Cache Hit` vs. `Cache Miss`):** Contiguous Stack data stays hot inside the CPU's L1/L2 Cache (like papers already on your desk), while scattered Heap allocations frequently miss the cache (like walking down to a warehouse for file `#402`).

4. **Real-Life Mimicry & Analogy Boundary:**
   - **Stack:** A barista's stack of uniform empty paper coffee cups — add on top, pull from top, never search or pull from the middle.
   - **Heap:** A luggage locker room — 4 bags go into Locker `#402` (`len: 4`, `cap: 4`) and you keep the claim ticket in your pocket (Stack). Return with 13 more bags, and the attendant moves all 17 bags to Locker `#815` (`cap: 20`), frees `#402`, and updates your ticket.
   - **Analogy Boundary (Where the Metaphor Breaks):** Popping the Stack does not physically erase old RAM bytes (the Stack Pointer moves back and future writes overwrite them), and Heap reallocation takes microseconds rather than walking — though repeating it inside a busy loop still hurts performance.

5. **`&str` vs. `String` and `Arrays`/`Tuples` vs. `Vec`:**
   - `&str` (`"Pavi"`): fixed text embedded in the program binary (zero Heap allocation).
   - `String` (`String::from("Pavi")`): growable, Heap-allocated text.
   - `Arrays` (`[i32; 3]`) and `Tuples` (`(i32, bool)`): fixed-size, stored on the **Stack**.
   - `Vec<T>` (`vec![1, 2, 3]`): growable list stored on the **Heap** *(we will dive deeper into `Vec` and `&str` vs. `String` in upcoming episodes)*.

6. **Cliffhanger — Double Free:**
   - When `}` hits, Rust frees `order`'s Heap locker. If two Stack variables held a ticket to the exact same Heap locker, both would try to free it at scope exit (**Double Free**). How Rust prevents this without a Garbage Collector is the focus of the next episode!

---

## 💻 Code Practice (Run in [Rust Playground](https://play.rust-lang.org/))

All runnable practice files for this episode live in [`./code/`](./code):

- [`code/stack_vs_heap.rs`](./code/stack_vs_heap.rs) — Full episode walkthrough: Stack scopes, `add_tax` Stack Frame, `String::from("Pavi")` + `.push_str(" + Extra Shot")`, `&str` vs. `String`, and Arrays/Tuples vs. `Vec`.
- [`code/reallocation_lab.rs`](./code/reallocation_lab.rs) — Live memory lab: inspect `size_of::<String>()` (`24 bytes`) and watch `order.as_ptr()` jump to a new Heap address when `capacity` overflows.
- [`code/memory_quiz.rs`](./code/memory_quiz.rs) — Interactive "Stack or Heap?" mini-quiz with prediction prompts and runnable answers.

### Quick Preview — Stack Scope & Heap Growth

```rust
fn add_tax(price: i32) -> i32 {
    let tax: i32 = 5;
    price + tax // Stack Frame pops right here!
}

fn main() {
    let total = add_tax(100);
    println!("Total on Stack: {total}");

    let mut order = String::from("Pavi");
    println!("Before -> len: {}, cap: {}", order.len(), order.capacity());

    order.push_str(" + Extra Shot");
    println!("After  -> len: {}, cap: {}", order.len(), order.capacity());
}
```

---

## 🎯 Your Action (CTA)

1. Open [`code/reallocation_lab.rs`](./code/reallocation_lab.rs) in [Rust Playground](https://play.rust-lang.org/) and **predict** whether `order.as_ptr()` will stay the same or change after `.push_str(" + Extra Shot")` before clicking **Run**.
2. Try [`code/memory_quiz.rs`](./code/memory_quiz.rs), write down your answers to all questions first, and share your score in the comments or [GitHub Discussions](https://github.com/some-of-cs/rust-for-everyone/discussions)!
