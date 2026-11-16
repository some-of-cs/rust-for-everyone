# B1 — Why the World Is Moving to Rust: Engineering & Business Behind the Decision

> **Outcome:** Evaluate Rust like a disciplined business investment using an **8-Step Decision Cycle** (`rust-decision-cycle`), real production case studies (Cloudflare, Tenable, Discord, Google Android, AWS Firecracker, ArtCraft), and honest payback math.

---

## 📊 The 8-Step Rust Decision Cycle

1. **Observe the Business Problem:** Separate costs into **Direct Cloud/Resource Cost**, **Security/Reliability Risk Cost**, and **Engineering Opportunity Cost** (chasing memory bugs vs. shipping features).
2. **Measure the Baseline:** Record unit economics on real traffic—cost per 1M requests/operations, `p95`/`p99` latency, error rate, and your Service Level Objective (`SLO`).
3. **Isolate the Bottleneck:** Profile first. If 80% of request time is spent waiting on an unindexed database query or external API, changing the application language will not solve it. Try simpler fixes (indexing, caching, autoscaling, algorithm fixes) first.
4. **Match the Bottleneck to a Rust Strength:** High CPU/memory footprint, garbage-collection tail-latency spikes, or memory-safety boundaries are strong fits. Remember: Safe Rust does not validate business logic, and any `unsafe` blocks require mandatory code review.
5. **Design a Bounded Pilot (Not a Full Rewrite):** Isolate a small, high-leverage component (worker service, sidecar, WebAssembly module, CLI binary, or shared core library) with identical load, identical `SLO`, and a fast rollback path.
6. **Validate with Comparable Production Evidence:** Compare your pilot against the baseline on the exact same workload and correctness criteria.
7. **Convert Technical Change into Business Value & Honest Payback:** Lower CPU usage on reserved servers is only *headroom* until you rightsize or decommission instances. Calculate `Payback = Migration Cost / Net Monthly Cash Savings`.
8. **Stop When Rust Is Not the Answer:** Issue an evidence-based verdict: `REJECT`, `NOT YET`, `GO (pilot)`, or `VALIDATED (scale)`.

---

## 🛠️ Use the `rust-decision-cycle` Skill & Prompt

We provide both a ready-to-paste bilingual prompt and a portable agent skill in [`./skill`](./skill):

- 📄 **[Ready-to-Use Prompt Card (`rust-decision-cycle-prompt.md`)](./skill/rust-decision-cycle-prompt.md)** — Fill in the brackets with your workload's metrics and run it in any LLM.
- 🤖 **[Agent Skill (`rust-decision-cycle/SKILL.md`)](./skill/rust-decision-cycle/SKILL.md)** — Installable skill definition with strict anti-hallucination and payback rules.
- 📈 **[Scorecard & Domain Fit Reference (`scorecard-and-fit.md`)](./skill/rust-decision-cycle/references/scorecard-and-fit.md)** — Detailed scoring rubric, domain boundaries, and LLM + Rust verification loop notes.

---

## 🎯 Your Action (CTA)
Profile your most expensive or highest-risk workload, fill in [`skill/rust-decision-cycle-prompt.md`](./skill/rust-decision-cycle-prompt.md), and define a bounded Rust pilot with clear stop criteria!
