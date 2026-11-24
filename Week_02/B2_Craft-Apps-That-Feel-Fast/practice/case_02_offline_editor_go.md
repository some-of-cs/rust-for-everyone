# Practice Case #2 — Offline-First Editor (`GO — Bounded Pilot`)

> **Goal:** Run the [`app-responsiveness-cycle`](../skill/app-responsiveness-cycle/SKILL.md) on this scenario and verify that it walks cleanly through **Step 1 to Step 8**, issues **`GO (Bounded Pilot)`**, keeps the existing UI intact, and labels the payback math **`ILLUSTRATIVE — NOT A FORECAST`**.

---

## 📋 Scenario Brief (Paste Into Your LLM With `app-responsiveness-cycle`)

```text
Run the `app-responsiveness-cycle` on the scenario below, explain technical terms plainly so both founders and engineers can follow, and return exactly one verdict: `REJECT`, `NOT YET`, `GO (Bounded Pilot)`, or `VALIDATED (Scale)`.

- App & Platforms: Offline-first note-taking and document editor running on iOS, Android, Web, and Desktop (existing Swift/Kotlin/React UIs).
- User & Business Pain: Power users with 50,000+ local notes experience 600 ms UI freezes on every search keystroke and occasional OOM crashes during local sync on mid-tier Android devices, driving paid subscription churn and recurring support tickets.
- Real-Device Baseline: Measured on mid-tier Android & iOS phones with a 50,000-note fixture — 600 ms keystroke latency, 41% dropped frames against the 16.6 ms (60 Hz) budget, P99 search response 680 ms, frequent GC memory spikes during local diff/sync.
- Profiling & Simpler Fixes Tried: Lists are already virtualized, components are memoized, and local search uses zero network calls. Client profiler proves 82% of main-thread frame time is spent in local full-text indexing, text diffing, and Garbage Collector pauses.
- Bounded Pilot Scope: Keep all existing UI layers untouched; extract ONLY the local search index and diff/sync engine into one shared Rust core accessed via batched FFI / UniFFI (mobile) and WebAssembly / Tauri (web/desktop).
- Safety, Rollback & Team: 2 engineers with Rust/FFI review policy in place; instant Feature-Flag rollback to the legacy search engine if parity or binary-size checks fail.
- Financials: Estimated pilot cost $18,000; estimated retained subscription revenue + saved support = $4,500/month (ILLUSTRATIVE — NOT A FORECAST).
```

---

## ✅ Expected Verdict & Self-Check Rubric

- **Expected Verdict:** **`GO (Bounded Pilot)`** (Passes **Steps 1–8**).
- **Architecture Rule Enforced:** Existing UI stays unchanged; only the hot local compute engine moves into an isolated **Shared Rust Core** behind batched `FFI` / `WASM` / `Tauri` calls with a **Feature Flag** rollback.
- **Real-Device Validation Plan (Step 6):** Benchmark before/after on the exact same mid-tier Android & iOS phones with the 50,000-note dataset, tracking `P99` frame drops, peak memory, battery impact, and **Binary Size** delta.
- **Payback Check (Step 7 — `ILLUSTRATIVE — NOT A FORECAST`):**
  $$\text{Payback} = \frac{\$18{,}000}{\$4{,}500/\text{month}} = 4\text{ months}$$
