# Practice Case #1 — E-Commerce Mobile App (`REJECT`)

> **Goal:** Run the [`app-responsiveness-cycle`](../skill/app-responsiveness-cycle/SKILL.md) on this scenario and verify whether your AI assistant (or your own review) stops cleanly at **Step 3** with **`REJECT`** instead of recommending a wasteful Rust rewrite.

---

## 📋 Scenario Brief (Paste Into Your LLM With `app-responsiveness-cycle`)

```text
Run the `app-responsiveness-cycle` on the scenario below, explain technical terms plainly so both founders and engineers can follow, and return exactly one verdict: `REJECT`, `NOT YET`, `GO (Bounded Pilot)`, or `VALIDATED (Scale)`.

- App & Platforms: React Native (iOS & Android) e-commerce shopping app with product catalog, search, and checkout.
- User & Business Pain: 1-star App Store / Play Store reviews complaining that the product catalog freezes while scrolling and users abandon checkout. Backend dashboards are all green (API responds in 40 ms).
- Real-Device Baseline: Measured on a mid-tier Android phone — 350 ms UI freezes while scrolling product lists, 28% dropped frames against the 16.6 ms (60 Hz) frame budget, P99 tap latency 420 ms, cold startup 3.1 s.
- Profiling & Simpler Fixes Tried: Client profiler shows two dominant bottlenecks: (1) the catalog API returns a 4 MB uncompressed JSON payload with no pagination, and (2) ProductCard items inside FlatList have no memoization and re-render on every scroll event. Neither pagination, Gzip compression, nor FlatList memoization has been tried yet.
- Bounded Pilot Scope: Team is debating rewriting the catalog parsing and filtering layer in Rust via FFI.
- Safety, Rollback & Team: 3 JavaScript/TypeScript engineers, 0 Rust engineers, no rollback plan defined.
- Financials: Estimated 6-week pilot cost $24,000; target payback horizon 6 months (ILLUSTRATIVE — NOT A FORECAST).
```

---

## ✅ Expected Verdict & Self-Check Rubric

- **Expected Verdict:** **`REJECT`** (Stopped at **Step 3 — Profile & Isolate / Simpler-Fixes Gate**).
- **Why Rust Won't Fix It:** A Rust core cannot shrink a `4 MB` uncompressed network payload or stop React Native from unnecessarily re-rendering unmemoized `FlatList` cards.
- **Prerequisite Fixes First:**
  1. Enable API **pagination** (e.g., 20–30 items per page) and **`Gzip`/`Brotli` compression**.
  2. Memoize list item components (`React.memo`) and configure proper list **virtualization** (`windowSize`, stable keys).
  3. Compress product thumbnails and cache images.
- **Re-evaluation Trigger:** Re-profile on the same mid-tier Android device **after** shipping pagination, compression, and list memoization. Only reopen Rust evaluation if heavy local CPU or GC pauses still blow past the `16.6 ms` frame budget.
