# Rust for Everyone (راست للجميع)

<div align="center">

> **Start with zero coding background, run Rust in your browser, and build toward a real portfolio service — or evaluate Rust for your engineering team with evidence-led business frameworks.**  
> Taught in everyday Egyptian Arabic with English technical terms and full English subtitles by **[Maher Aladdin](https://github.com/maheraladdin)**.

[![Some(CS) Org](https://img.shields.io/badge/Org-Some(CS)-0F2942.svg)](https://github.com/some-of-cs)
[![Rust Playground](https://img.shields.io/badge/Run%20in-Rust%20Playground-DEA584.svg?logo=rust)](https://play.rust-lang.org/)
[![Community Discussions](https://img.shields.io/badge/Community-GitHub%20Discussions-8A2BE2.svg?logo=github)](https://github.com/some-of-cs/rust-for-everyone/discussions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Sponsor](https://img.shields.io/badge/Sponsor-%E2%99%A5-ff69b4.svg)](https://github.com/sponsors/maheraladdin)

[Learning Paths](#-choose-your-path) •
[Curriculum & Weekly Folders](#-curriculum--learning-blocks) •
[Learning Timeline](TIMELINE.md) •
[B1 Decision Skill](./Week_01/B1_Why-the-World-Is-Moving-to-Rust/skill) •
[Community Discussions](#-community--discussions) •
[Sponsors](#-backers--corporate-sponsors)

</div>

---

## 🧭 Choose Your Path

**Rust for Everyone** is designed around two connected tracks so both learners and technical decision-makers get concrete, verifiable outcomes:

| Path | Who It Serves | Episode Track | What You Will Achieve |
| :--- | :--- | :--- | :--- |
| **🛠️ Build with Rust** | People with no programming background & junior developers revisiting foundations | **S1 – S24** *(in order; B episodes add industry context)* | Predict, change, and explain small Rust programs in the browser (`S1–S6`), move to local Cargo projects (`S7+`), and complete a guided service with tests and documentation. |
| **📊 Evaluate Rust** | Managers, founders, CTOs, recruiters, and engineers making an adoption decision | **B1 – B12** *(S episodes are optional)* | Explain relevant engineering/business tradeoffs and outline a bounded pilot with baseline measurements, payback math, and stop criteria (`REJECT`, `NOT YET`, `GO (pilot)`, `VALIDATED (scale)`). |

### How Every Explaining Lesson (`S`) Works
Every technical episode follows a visible three-step learning loop:
1. **Concept (الفكرة):** Understand the mental model first using everyday analogies before syntax.
2. **Summary (الملخص):** Consolidate the core engineering rules and where the analogy breaks.
3. **Code Practice (التطبيق العملي):** Change one value, **predict the output**, run the code in [Rust Playground](https://play.rust-lang.org/), and explain the result.

---

## 📚 Curriculum & Learning Blocks

> **Note:** Each `Week_XX` folder groups a self-paced **learning block**. See **[`TIMELINE.md`](TIMELINE.md)** for the full milestone and checkpoint progression.

| Block / Folder | Theme | Explaining Rust (`S` Track) | Business Case (`B` Track) |
| :--- | :--- | :--- | :--- |
| **[`Week_01`](./Week_01)** | **Start from zero** | **[S1](./Week_01/S1_What-Code-Is):** What Coding Means, Compilers & Your First Program<br>**[S2](./Week_01/S2_Variables-Types-Decisions):** Variables, Types, Shadowing & Decisions | **[B1](./Week_01/B1_Why-the-World-Is-Moving-to-Rust):** Why the World Is Moving to Rust *(+ [`rust-decision-cycle`](./Week_01/B1_Why-the-World-Is-Moving-to-Rust/skill))* |
| **[`Week_02`](./Week_02)** | **Making things happen** | **[S3](./Week_02/S3_Operators-Loops-Functions):** Operators, `if/else if`, Loops & Functions<br>**S4:** How a Computer Remembers Things | **B2:** Craft Apps That Feel Fast *(+ `app-responsiveness-cycle`)* |
| **[`Week_03`](./Week_03)** | **The big idea: Ownership** | **S5:** Ownership — One Thing, One Owner<br>**S6:** Borrowing — Looking Without Taking | **B3:** Garbage Collection & the Discord Story |
| **[`Week_04`](./Week_04)** | **Real projects & the cost of bugs** | **S7:** Install Rust & Meet Cargo<br>**S8:** Text & Data (`String` vs `&str`) | **B4:** The Real Cost of Bugs |
| **[`Week_05`](./Week_05)** | **Modeling the world** | **S9:** Structs — Your Own Blueprints<br>**S10:** Enums, `Option` & `match` | **B5:** Should You Actually Use Rust? |
| **[`Week_06`](./Week_06)** | **Everyday data** | **S11:** Collections — Lists & Maps<br>**S12:** When Things Go Wrong (`Result`) | **B6:** The Cloud Bill & Energy |
| **[`Week_07`](./Week_07)** | **Organizing code** | **S13:** Modules, Crates & Tests<br>**S14:** Traits, Generics & Lifetimes | **B7:** Rust at Web Scale |
| **[`Week_08`](./Week_08)** | **Many things at once** | **S15:** Concurrency — Threads & Safe Sharing<br>**S16:** Async — Just Enough | **B8:** Rust vs Go, Node, Python |
| **[`Week_09`](./Week_09)** | **Talking to the world** | **S17:** JSON & Serde<br>**S18:** Build a Web Service | **B9:** The Rust Community & Open Source |
| **[`Week_10`](./Week_10)** | **Serverless & shipping** | **S19:** Harden Your Service<br>**S20:** Cargo Lambda — Serverless in Rust | **B10:** Serverless Economics |
| **[`Week_11`](./Week_11)** | **Put it in your portfolio** | **S21:** Capstone — Build<br>**S22:** Capstone — Ship | **B11:** The Adoption Playbook |
| **[`Week_12`](./Week_12)** | **What's next** | **S23:** Smart Pointers & the Ecosystem<br>**S24:** Your Learning Path & Portfolio | **B12:** The Big Picture & What's Next |

---

## 💬 Community & Discussions

This repository is the home and discussion hub for **Rust for Everyone** learners and engineering leaders:

1. **Practice in the Browser (`S1–S6`):** Open any episode folder, copy the starter code into [Rust Playground](https://play.rust-lang.org/), change a value, **write down your prediction before pressing Run**, and compare your prediction with the actual output.
2. **Evaluate Workloads (`B1–B12`):** Use the portable AI skills and bilingual prompts inside the `B` folders (starting with [`Week_01/B1_Why-the-World-Is-Moving-to-Rust/skill`](./Week_01/B1_Why-the-World-Is-Moving-to-Rust/skill)) to evaluate your own workloads using real metrics.
3. **Join the Conversation:** Head over to **[GitHub Discussions](https://github.com/some-of-cs/rust-for-everyone/discussions)** or **[Issues](https://github.com/some-of-cs/rust-for-everyone/issues)** to ask questions about compiler diagnostics (`E0384`, borrow checker rules, etc.), share your boundary test predictions, or discuss bounded Rust pilots.

---

## 💖 Backers & Corporate Sponsors

`rust-for-everyone` is an open-access educational and engineering product by **[Some(CS)](https://github.com/some-of-cs)**. Corporate and individual sponsorship directly finances free Arabic/English systems engineering lessons, runnable code labs, and open-source engineering decision tools.

<div align="center">

[![Become a Sponsor](https://img.shields.io/badge/Become%20a%20Sponsor-%E2%99%A5-ea4aaa?style=for-the-badge&logo=github-sponsors)](https://github.com/sponsors/maheraladdin)

</div>

### Corporate Sponsorship Tiers & Benefits

| Tier | Monthly | Placement & Benefits |
| :--- | :--- | :--- |
| **🌟 Visionary** | $1,000 / mo | Large logo on `README.md` Sponsor Wall • Dedicated mention in curriculum materials • Scoped team onboarding Q&A session |
| **🥇 Gold Partner** | $500 / mo | Prominent logo on `README.md` Sponsor Wall • Highlighted in learning block releases & community discussions |
| **🥈 Silver Backer** | $150 / mo | Company logo, link, and mention on the `README.md` Sponsor Wall |
| **☕ Individual** | $10 – $50 / mo | Listed on the Individual Backers Wall and recognized in the community |

### Sponsors & Backers Wall

<div align="center">

| 🌟 Visionary Sponsors | 🥇 Gold Partners | 🥈 Silver Backers | ☕ Individual Backers |
| :---: | :---: | :---: | :---: |
| *Your company logo here*<br>[Become a Visionary Sponsor](https://github.com/sponsors/maheraladdin) | *Your company logo here*<br>[Become a Gold Partner](https://github.com/sponsors/maheraladdin) | *Your company logo here*<br>[Become a Silver Backer](https://github.com/sponsors/maheraladdin) | [Join our Individual Backers](https://github.com/sponsors/maheraladdin) |

</div>

> [!TIP]
> **Need an Invoice or Team Rust Onboarding?**  
> We support formal corporate sponsorship invoicing and scoped team onboarding workshops. Reach out via [GitHub Discussions](https://github.com/some-of-cs/rust-for-everyone/discussions) or [Some(CS)](https://github.com/some-of-cs).

---

## License

Released under the [MIT License](LICENSE). Created and maintained by **[Maher Aladdin](https://github.com/maheraladdin)** & **[Some(CS)](https://github.com/some-of-cs)**.
