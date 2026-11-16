# Scorecard, Domain Fit & Reference Guide (`rust-decision-cycle`)

## 1. Verdict Decision Matrix

| Verdict | When to Issue | Required Output |
| :--- | :--- | :--- |
| **`REJECT`** | • Bottleneck is in DB queries, missing indexes, or external network I/O<br>• Basic optimizations (caching, indexing, autoscaling, algorithm fix) have not been tried<br>• Early-stage startup still searching for product-market fit where cloud cost is negligible | • Stop at Step 3 (do not design a pilot)<br>• List the exact simpler fixes to run first<br>• Define the exact condition to reopen the decision |
| **`NOT YET`** | • Technical bottleneck looks plausible for Rust, **but** one or more of the 4 Readiness Gates is missing (`Unknown` baseline, no profiling, no isolated boundary, no rollback, or no owner/reviewer) | • Identify which of the 4 Readiness Gates failed<br>• Specify the exact measurement or team step needed to reach `GO (pilot)` |
| **`GO (pilot)`** | • Profiling confirms CPU, memory, GC tail-latency, or memory-safety bottleneck in owned code<br>• Simpler fixes were tried and plateaued<br>• All 4 Readiness Gates pass, and break-even math is realistic | • Define the bounded pilot scope, SLO parity check, rollback mechanism, and required monthly net savings to break even within the decision horizon |
| **`VALIDATED (scale)`** | • Pilot has been executed on comparable production load<br>• Output correctness and SLO compliance are verified<br>• Instance rightsizing / decommissioning yields verified net monthly savings | • Report measured payback period, decommissioning checklist (turning headroom into cash savings), and production rollout guardrails |

---

## 2. Software Domain Fit & Boundaries

| Domain | High-Leverage Rust Boundary | When Rust Is the Wrong Tool |
| :--- | :--- | :--- |
| **1. Backend & Network Services** | High-throughput proxies, data-ingestion workers, latency-sensitive services where GC pauses violate SLOs, or memory-billed serverless functions (e.g., Axum services). | CRUD services spending >70% of request time waiting on unindexed SQL queries or slow upstream APIs. |
| **2. Web (Browser via WebAssembly)** | Isolating heavy client-side compute (image/video processing, canvas editors, cryptography, parsing) into a Wasm module alongside JavaScript/TypeScript. | Rewriting standard marketing pages or simple forms that have no client compute bottleneck. |
| **3. Full-Stack Web in Rust** | Teams that already have Rust expertise sharing types across client and server (e.g., Leptos). | Teams that need to hire 10 frontend developers tomorrow in a market dominated by React/Next.js/Node.js, or need rapid UI iteration for product-market fit. |
| **4. Desktop & Mobile Apps** | Native lightweight desktop shells (Tauri 2) or shared core Rust libraries linked into native Swift (iOS) and Kotlin (Android) UIs; AI-assisted native app suites (such as ArtCraft's 13 native creative/office apps). | Forcing every UI screen to be written in Rust when a native or web UI layer over a Rust core is faster to maintain. |
| **5. CLI, Embedded, OS & DevTools** | Single-binary zero-dependency CLI tools, microVMs (e.g., AWS Firecracker), embedded firmware, and database engines requiring deterministic resource control. | Throwaway internal scripts where a 20-line shell or Python script suffices. |
| **6. Data, Media, Gaming & AI Engines** | High-speed audio/video/image pipelines, game engines, and AI inference/tokenization cores exposed to Python, Node.js, or native apps. | Replacing high-level data-science exploration notebooks that only orchestrate existing C/Rust libraries. |

---

## 3. Working with LLMs & Rust (The Compiler as Automated Gatekeeper)

- **Why Rust + LLMs Works Well:** In languages like C++ or Python, LLM-generated code can look convincing while hiding memory leaks, null/dangling pointers, or data races that only fail in production. In Safe Rust, the compiler, ownership system, and borrow checker reject memory and concurrency errors at compile time with actionable diagnostics—creating a fast, deterministic self-correction loop for the LLM.
- **What the Compiler Does NOT Catch:**
  1. **Business Logic Errors:** Calculating `paid + total` instead of `paid - total` compiles cleanly.
  2. **Performance Anti-Patterns (`.clone()` Sprawl):** LLMs frequently insert `.clone()` or extra allocations to bypass borrow-checker errors, silently degrading throughput and memory efficiency. Human code review and profiling remain mandatory.
  3. **`unsafe` Escapes:** Any `unsafe` block disables memory-safety guarantees and requires strict human review.

---

## 4. Honest Financial & Payback Formulas

1. **Baseline Monthly Cost:**
   $$C_{\text{baseline}} = N_{\text{instances}} \times C_{\text{instance}}$$
2. **Pilot Projected Monthly Gross Savings:**
   $$S_{\text{gross}} = (N_{\text{baseline}} - N_{\text{pilot}}) \times C_{\text{instance}}$$
3. **Net Monthly Cash Savings (Only after decommissioning/rightsizing):**
   $$S_{\text{net}} = S_{\text{gross}} - C_{\text{extra\_ops}}$$
4. **Payback Period (Months):**
   $$\text{Payback} = \frac{C_{\text{migration}}}{S_{\text{net}}}$$
5. **Required Monthly Savings for Target Horizon $H$:**
   $$S_{\text{required}} = \frac{C_{\text{migration}}}{H}$$
