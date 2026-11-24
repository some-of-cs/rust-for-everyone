# Scorecard, Client Platform Fit & Founder Glossary (`app-responsiveness-cycle`)

## 1. Verdict Decision Matrix

| Verdict | When to Issue | Required Output |
| :--- | :--- | :--- |
| **`REJECT`** | • Bottleneck is oversized API payloads (`4 MB` uncompressed JSON), missing pagination/compression, unmemoized UI re-renders, missing list virtualization, or unoptimized images<br>• Simple CRUD app with no measurable client CPU/GC bottleneck | • Stop at **Step 3** (do not design a Rust pilot)<br>• List the exact UI/network fixes to ship first<br>• State the exact re-profiling condition to reopen Rust evaluation |
| **`NOT YET`** | • Client-side CPU, parsing, or GC bottleneck looks plausible, **but** real-device baseline, profiler trace, isolated core boundary, feature-flag rollback, or code owner is `"Unknown"` or missing | • Name the exact missing prerequisite(s)<br>• Specify the concrete measurement or team step needed to reach `GO (Bounded Pilot)` |
| **`GO (Bounded Pilot)`** | • Real-device profiling confirms heavy local CPU, parsing/indexing/diffing, or GC-pause freezes<br>• Simpler UI/network fixes were exhausted<br>• Bounded Rust core + batched `FFI`/`WASM`/`Tauri` bridge + instant Feature-Flag rollback + realistic break-even payback are defined | • Define the isolated core slice (keep existing UI)<br>• Specify target mid/low-tier test devices, parity checks, rollback switch, and break-even monthly benefit |
| **`VALIDATED (Scale)`** | • Bounded pilot has already been measured on identical real target devices and dataset<br>• `P99` frame drops dropped meaningfully with acceptable binary size, memory, and battery impact | • Report verified before/after device metrics, measured payback period, and rollout guardrails |

---

## 2. Client Platform Fit & Isolation Boundaries

| Platform / Stack | High-Leverage Rust Core Boundary | When Rust Is the Wrong Tool |
| :--- | :--- | :--- |
| **Mobile (`iOS` Swift / `Android` Kotlin)** | Shared Rust core via **`FFI` / `UniFFI`** for offline sync, local full-text search, media/file parsing, compression, or crypto. | Rewriting native UI screens or fixing slow network calls / unindexed backend queries. |
| **Cross-Platform Mobile (`React Native` / `Expo` / `Flutter`)** | Offloading heavy CPU parsing, local indexing, or image/audio transforms to a batched native Rust module so the JS/Dart UI thread stays under `16.6 ms`. | Fixing unmemoized `FlatList` re-renders, massive uncompressed JSON payloads, or chatty per-row bridge calls. |
| **Web (`React` / `Next.js` / Browser Apps)** | **`WebAssembly (WASM)`** inside a Web Worker for client-side editors, spreadsheets, CAD/canvas math, parsers, or local search. | Standard marketing pages, simple CRUD dashboards, or forms with no client-side compute bottleneck. |
| **Desktop (`Tauri` / Native Shells)** | Lightweight **`Tauri`** shell + Rust backend core for local-first editors, developer tools, and file-heavy apps with low RAM footprint. | Apps whose latency is caused purely by slow remote APIs rather than local CPU or memory pressure. |

---

## 3. Plain-Language Glossary for Non-Technical Founders

| Technical Term | Plain-Language Meaning |
| :--- | :--- |
| **`16.6 ms` Frame Budget (`60 Hz`)** | A phone screen redraws 60 times per second (`1000 / 60 = 16.6 ms`). Per [Google RAIL](https://web.dev/articles/rail), the OS uses `~6 ms` to paint, leaving your app `~10 ms` before scrolling stutters (`8.3 ms` total at `120 Hz`). |
| **`Main-Thread Blocking`** | One heavy task hogging the single lane that draws the screen—like a 1,000-item bulk order freezing the only supermarket cashier. |
| **`Profiling` / `Flamegraph`** | A "speed X-ray" showing exactly which part of the app is consuming milliseconds on a real phone. |
| **`Payload` / `Pagination` / `Gzip`** | The size of data sent by the server; splitting it into small pages (`Pagination`) and compressing it (`Gzip`) fixes network lag in a day. |
| **`UI Re-renders` & `Virtualization`** | Redrawing screen cards unnecessarily; `Virtualization` renders only items currently visible on screen instead of thousands off-screen. |
| **`Garbage Collector (GC)`** | Automatic memory cleanup in managed runtimes that can briefly pause the app on budget phones, causing sudden `P99` stutter spikes. |
| **`FFI` / `WebAssembly` / `Tauri`** | Bridges that let your existing UI talk to a small, fast Rust core without rewriting the whole app. |
| **`Feature Flag` & `Rollback`** | A safety switch that lets you revert users to the previous code path in seconds if the pilot regresses. |
| **`Binary Size`** | The download size of the app in the App Store / Play Store. |

---

## 4. Honest Payback Formulas

1. **Required Monthly Benefit for Target Horizon $H$ (months):**
   $$B_{\text{required}} = \frac{C_{\text{pilot}}}{H}$$
2. **Post-Pilot Payback Period (months):**
   $$\text{Payback} = \frac{C_{\text{pilot}}}{\text{Monthly Retained Revenue} + \text{Monthly Saved Support} - \text{Extra Monthly Maintenance}}$$

> Always label worked examples with hypothetical inputs **`ILLUSTRATIVE — NOT A FORECAST`**.
