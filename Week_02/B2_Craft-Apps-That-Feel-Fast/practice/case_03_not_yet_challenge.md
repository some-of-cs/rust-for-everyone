# Practice Case #3 (Challenge) — Media Chat App (`NOT YET`)

> **Goal:** Test whether you (and your AI assistant) catch missing real-device measurements and return **`NOT YET`** instead of guessing numbers or jumping straight to `GO`.

---

## 📋 Scenario Brief (Paste Into Your LLM With `app-responsiveness-cycle`)

```text
Run the `app-responsiveness-cycle` on the scenario below, explain technical terms plainly so both founders and engineers can follow, and return exactly one verdict: `REJECT`, `NOT YET`, `GO (Bounded Pilot)`, or `VALIDATED (Scale)`.

- App & Platforms: Cross-platform Flutter messaging app with encrypted voice notes and media previews.
- User & Business Pain: Several users on budget Android phones reported sluggish scrolling and occasional freezes when opening group chats with many voice notes.
- Real-Device Baseline: Only tested on the lead engineer's MacBook Pro and iPhone 16 Pro (smooth 120 fps). Mid/low-tier Android frame times, P99 latency, and dropped frame rate are Unknown.
- Profiling & Simpler Fixes Tried: No client profiler trace or flamegraph has been captured yet. Team suspects audio waveform decoding or decryption might be blocking the main thread, but hasn't verified whether image caching or list virtualization is the real culprit.
- Bounded Pilot Scope: Possibly moving audio decoding into Rust via FFI, not yet scoped.
- Safety, Rollback & Team: No Feature Flag rollback plan or FFI code owner assigned yet.
- Financials: Pilot cost and monthly business impact are Unknown.
```

---

## ✅ Expected Verdict & Self-Check Rubric

- **Expected Verdict:** **`NOT YET`**
- **Why Not `GO` Yet?**
  1. **Zero real-device baseline:** Flagship developer hardware does not satisfy Step 2.
  2. **Zero profiler evidence:** Without a trace on a budget Android device (Step 3), the team cannot confirm whether the bottleneck is media decoding/CPU or simple image/list rendering.
  3. **Missing pilot readiness gates (Step 5):** No isolated core boundary, no Feature-Flag rollback, and no assigned FFI owner.
- **Concrete Next Step:** Capture a real profiler trace and frame-time baseline on a representative budget Android device, test simple list/image fixes first, and re-run `app-responsiveness-cycle` with verified measurements.
