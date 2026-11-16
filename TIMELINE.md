# Rust for Everyone — Learning & Checkpoints Timeline

This timeline outlines the step-by-step progression across all 12 learning blocks of **Rust for Everyone**. Move through the blocks at your own pace—every checkpoint defines what you can **predict, build, or evaluate** before moving forward.

---

## Phase 1: Browser Foundations (`Week_01` – `Week_03`)
*Environment: [Rust Playground](https://play.rust-lang.org/) in your browser — no installation required.*

| Block | Episodes | Learner & Decision Checkpoints | Core Concepts Covered |
| :--- | :--- | :--- | :--- |
| **[`Week_01`](./Week_01)** | **S1, S2, B1** | • Explain `Problem → Source → Build → Run` and distinguish compiler checks from logic bugs (`S1`)<br>• Use `let`, `let mut`, `const`, and `Shadowing` (mutate then freeze), and predict `if/else` boundary conditions (`S2`)<br>• Run the 8-step `rust-decision-cycle` on a real workload (`B1`) | Data vs. Algorithm, Compilers vs. Interpreters, Language Levels, Type Inference, Immutability, Shadowing, Booleans, Unit Economics & Pilot Design |
| **[`Week_02`](./Week_02)** | **S3, S4, B2** | • Automate repeated actions with loops and organize reusable code into functions (`S3`)<br>• Build an accurate mental model of how a computer stores and remembers values (`S4`)<br>• Evaluate client/app responsiveness bottlenecks with `app-responsiveness-cycle` (`B2`) | Loops, Functions, Parameters & Return Values, Stack vs. Heap Intuition, UI/App Responsiveness |
| **[`Week_03`](./Week_03)** | **S5, S6, B3** | • Explain a **move** versus a **borrow** in a small browser example (`S5–S6`)<br>• Understand the first lifetimes intuition: a reference cannot outlive what it refers to (`S6`)<br>• Evaluate Garbage Collection tail-latency tradeoffs (`B3`) | Ownership (One Thing, One Owner), Borrowing (`&` and `&mut`), GC Pause Tradeoffs |

---

## Phase 2: Real Projects, Data Modeling & Error Handling (`Week_04` – `Week_07`)
*Environment: Local Rust toolchain (`rustup`, `cargo`, `rustc`) introduced in `S7`.*

| Block | Episodes | Learner & Decision Checkpoints | Core Concepts Covered |
| :--- | :--- | :--- | :--- |
| **[`Week_04`](./Week_04)** | **S7, S8, B4** | • Install Rust, create and run local projects with `cargo` (`S7`)<br>• Use owned text (`String`) and borrowed text slices (`&str`) and observe elided lifetime signatures (`S8`)<br>• Quantify the operational and security cost of memory bugs (`B4`) | Cargo Workflow, `String` vs. `&str`, Lifetimes Touch 2 (Elision), Cost of Memory Vulnerabilities |
| **[`Week_05`](./Week_05)** | **S9, S10, B5** | • Model domain records using `struct` blueprints (`S9`)<br>• Represent optional values and state transitions safely using `enum`, `Option<T>`, and `match` (`S10`)<br>• Assess organizational readiness for Rust (`B5`) | Structs, Methods, Enums, `Option<T>`, Exhaustive Pattern Matching |
| **[`Week_06`](./Week_06)** | **S11, S12, B6** | • Store and query multiple records using `Vec<T>` and `HashMap<K, V>` (`S11`)<br>• Handle recoverable failures explicitly using `Result<T, E>` (`S12`)<br>• Connect compute/memory efficiency to actual cloud bill savings (`B6`) | Collections (`Vec`, `HashMap`), Explicit Error Handling (`Result`), Cloud Rightsizing & Energy |
| **[`Week_07`](./Week_07)** | **S13, S14, B7** | • Organize code into modules, write automated unit tests (`cargo test`), and read one trait bound and one annotated lifetime relationship (`S13–S14`)<br>• Evaluate Rust at high-throughput web scale (`B7`) | Modules, Crates, Unit Testing, Traits, Generics, Explicit Lifetimes |

---

## Phase 3: Concurrency, Web Services & Portfolio Capstone (`Week_08` – `Week_12`)
*Goal: Build, test, harden, and document a reproducible Workshop Registration Web Service for your portfolio.*

| Block | Episodes | Learner & Decision Checkpoints | Core Concepts Covered |
| :--- | :--- | :--- | :--- |
| **[`Week_08`](./Week_08)** | **S15, S16, B8** | • Distinguish parallel work across threads from waiting with `async/await` and run provided examples (`S15–S16`)<br>• Compare Rust alongside Go, Node.js, and Python (`B8`) | Threads, Safe Sharing, Async/Await Basics, Multi-Language Architecture Fit |
| **[`Week_09`](./Week_09)** | **S17, S18, B9** | • Serialize and deserialize JSON payloads with `Serde` (`S17`)<br>• Understand HTTP requests, responses, and status codes, and build a local web service (`S18`)<br>• Navigate the open-source Rust ecosystem (`B9`) | JSON & Serde, HTTP Fundamentals, Axum Web Service, Open-Source Governance |
| **[`Week_10`](./Week_10)** | **S19, S20, B10** | • Validate user input and test both success and error HTTP paths (`S19`)<br>• Explore optional serverless packaging with `Cargo Lambda` (`S20`)<br>• Model cold starts, execution duration, and memory pricing in serverless (`B10`) | Input Validation, Service Hardening, Cargo Lambda, Serverless Unit Economics |
| **[`Week_11`](./Week_11)** | **S21, S22, B11** | • Build, test, and document the complete **Workshop Registration Service** capstone project (`S21–S22`)<br>• Design a step-by-step team adoption and review playbook (`B11`) | Portfolio Service Architecture, Reproducible Documentation, Team Onboarding Playbook |
| **[`Week_12`](./Week_12)** | **S23, S24, B12** | • Understand smart pointers (`Box`, `Rc`, `Arc`) and where they fit in the ecosystem (`S23`)<br>• Present your completed portfolio project and choose your next specialization path (`S24`, `B12`) | Smart Pointers, Ecosystem Navigation, Next Engineering Milestones |
