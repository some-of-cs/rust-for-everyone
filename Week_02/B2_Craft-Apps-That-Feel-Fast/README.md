# B2 — Craft Apps That Feel Fast: Diagnosing Client & App Responsiveness

> **Outcome:** Diagnose UI jank, dropped frames (`16.6 ms` at 60 Hz / `8.3 ms` at 120 Hz), main-thread blocking, `P99` latency spikes, and mobile/desktop startup lag using the **8-Step App Responsiveness Decision Cycle** (`app-responsiveness-cycle`), grounded in [Google RAIL](https://web.dev/articles/rail) and [Google/Deloitte's *Milliseconds Make Millions*](https://web.dev/case-studies/milliseconds-make-millions) — with **zero coding required**.

---

## 📊 Why Milliseconds Matter (Evidence)

- **Frame Budget ([Google RAIL](https://web.dev/articles/rail)):** A standard `60 Hz` mobile screen refreshes every **`16.6 ms`** (`8.3 ms` at `120 Hz`). Because the OS/browser needs `~6 ms` to paint and composite each frame, your app code has **`<= 10 ms`** at `60 Hz` before frames drop and scrolling stutters.
- **Revenue Impact ([Google & Deloitte — *Milliseconds Make Millions*](https://web.dev/case-studies/milliseconds-make-millions)):** Across **37 brands** and **30M+ mobile sessions** ([Deloitte Ireland](https://www.deloitte.com/ie/en/services/consulting/research/milliseconds-make-millions.html)), a **`0.1 s` (`100 ms`)** mobile speed improvement correlated with **`+8.4%`** retail conversion rate, **`+9.2%`** average order value (AOV), and **`+10.1%`** travel conversion rate.

---

## 🧭 The 8-Step App Responsiveness Cycle (`app-responsiveness-cycle`)

1. **Observe User-Perceived Pain:** Classify real customer pain into **Churn & Rating Impact** (1-star reviews, checkout abandonment), **Perceived Friction** (UI freezes, stuttering scroll), and **Team Time Drag** (weekly lag-bug firefighting).
2. **Measure the Real-Device Baseline:** Measure on actual mid/low-tier user phones (never only a developer MacBook Pro): frame time vs. `16.6 ms`, dropped frame rate, worst-case (`P99`) button response time, and cold app launch time.
3. **Profile & Isolate the Bottleneck (Simpler-Fixes Gate):** Run a client profiler ("speed X-ray") to see where the `16.6 ms` budget actually goes. If oversized API payloads (`4 MB JSON`), missing pagination/compression, or unnecessary UI re-renders dominate, **STOP here (`REJECT`)** and fix those first.
4. **Match the Bottleneck to Rust's Client Strengths:** Confirm the profiled bottleneck is heavy local CPU computation, deterministic zero-GC parsing/indexing/search/diffing, or a shared cross-platform core (`iOS`, `Android`, `Web`, `Desktop`).
5. **Design a Bounded, Isolated Pilot (Not a Full UI Rewrite):** Keep your existing UI (`Swift`, `Kotlin`, `React Native/Expo`, `Flutter`, `React`) and isolate **one** hot-path Rust core behind a batched bridge (`FFI` / `UniFFI`, `WebAssembly`, or `Tauri`) with an instant **Feature Flag** rollback.
6. **Validate on Real Target Devices:** Benchmark the Rust-core pilot against the legacy baseline on the exact same mid/low-tier devices and dataset, checking `P99` frame drops, memory/battery stability, and app download size (`Binary Size`).
7. **Convert Milliseconds into Business Value & Honest Payback:** Calculate `Payback = Pilot Cost / (Monthly Retained Revenue + Saved Support Cost)` and label any hypothetical example `ILLUSTRATIVE — NOT A FORECAST`.
8. **Issue One Binding Verdict:** `REJECT`, `NOT YET`, `GO (Bounded Pilot)`, or `VALIDATED (Scale)`.

---

## 🛡️ Strategic Boundary: When NOT to Use Rust

Fix these simpler, faster bottlenecks **before** introducing a Rust core:
- Unmemoized list re-renders, missing list virtualization, or uncompressed images.
- Oversized API responses (e.g., `4 MB` uncompressed JSON without pagination or `Gzip`/`Brotli`).
- Standard `CRUD` forms and screens with no measurable client-side CPU or GC bottleneck.
- Any proposal to rewrite the entire UI from scratch in Rust instead of isolating the hot compute core.

---

## 🛠️ Use the `app-responsiveness-cycle` Skill & Prompt

We provide both a ready-to-paste bilingual prompt and a portable AI agent skill in [`./skill`](./skill):

- 📋 **[Ready-to-Use Prompt Card (`app-responsiveness-cycle-prompt.md`)](./skill/app-responsiveness-cycle-prompt.md)** — Fill in the brackets with your real-device metrics and run it in any LLM.
- 🤖 **[Agent Skill (`app-responsiveness-cycle/SKILL.md`)](./skill/app-responsiveness-cycle/SKILL.md)** — Installable skill definition with strict anti-hallucination, Step 3 early-stop, and honest payback rules.
- 📊 **[Scorecard & Client Fit Reference (`scorecard-and-fit.md`)](./skill/app-responsiveness-cycle/references/scorecard-and-fit.md)** — Verdict matrix, platform integration boundaries (`FFI` / `WASM` / `Tauri`), plain-language glossary for founders, and payback formulas.

---

## 🧪 Hands-On Practice Cases (`./practice`)

Test the `app-responsiveness-cycle` on three realistic startup scenarios with self-check rubrics:

- ❌ **[Case #1 (`REJECT`) — E-Commerce Mobile App (`case_01_ecommerce_reject.md`)](./practice/case_01_ecommerce_reject.md):** `350 ms` scroll freezes from `4 MB` uncompressed JSON payloads and unmemoized `FlatList` cards (stops cleanly at Step 3).
- ✅ **[Case #2 (`GO — Bounded Pilot`) — Offline-First Editor (`case_02_offline_editor_go.md`)](./practice/case_02_offline_editor_go.md):** `600 ms` local search freezes across 50,000 notes (`82%` CPU indexing/diffing + GC pauses) solved via an isolated Rust core (`FFI`/`WASM`/`Tauri`).
- ⏸️ **[Case #3 (`NOT YET` Challenge) — Media Chat App (`case_03_not_yet_challenge.md`)](./practice/case_03_not_yet_challenge.md):** Spot the missing mid-tier Android baseline and profiler trace before jumping to a Rust pilot.

---

## 🎯 Your Action (CTA)
Profile your slowest mobile or desktop screen on a real mid-tier device, fill in [`skill/app-responsiveness-cycle-prompt.md`](./skill/app-responsiveness-cycle-prompt.md), and share your verdict (`REJECT` vs. `GO`) in [GitHub Discussions](https://github.com/some-of-cs/rust-for-everyone/discussions)!
