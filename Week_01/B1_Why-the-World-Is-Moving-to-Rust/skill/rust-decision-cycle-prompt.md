# Rust Decision Cycle — Ready-to-Use Prompt (نموذج قرار Rust الجاهز)

Use the Arabic or English prompt template below to evaluate whether a workload should adopt Rust, run a bounded pilot, wait for prerequisites, or reject the rewrite.

---

## 🇪🇬 النسخة العربية (Arabic Prompt Card)

```text
شغّل دورة rust-decision-cycle على السيناريو ده، ورجّع لي قرار واحد: REJECT أو NOT YET أو GO (pilot) أو VALIDATED (scale).

- الشغل: [الخدمة أو المنتج بيعمل إيه، في جملة أو اتنين]
- الألم: [إيه اللي بيوجع: فلوس، ولا خطر، ولا وقت الفريق؟ وبكام أو قد إيه؟]
- الـ baseline: [الأرقام اللي اتقاست على الـ traffic الحقيقي: الـ p95، وتكلفة الوحدة الواحدة، ونسبة الأخطاء، والـ SLO]
- الـ bottleneck: [الـ profiling بيقول الوقت أو الذاكرة رايحين فين؟ وإيه الحلول الأبسط اللي جرّبتها؟]
- الحدود: [الجزء اللي نقدر نجرّبه لوحده: service، ولا sidecar، ولا Wasm module، ولا مكتبة مشتركة؟]
- الفريق: [مين هيمتلك الكود ومين هيراجعه؟ وفيه سياسة للـ unsafe؟]
- الفلوس: [تكلفة الانتقال التقديرية، وأفق القرار بالشهور، وتكلفة التشغيل الزيادة لو معروفة]
- التحقق والـ rollback: [إزاي هنتأكد إن النتايج صح والـ SLO ماتأثّرش؟ وإزاي نرجع للقديم؟]

قواعد: اللي مش معروف اكتبه «مش معروف». وما تخترعش أرقام. لو الأرقام افتراضية، اكتب جنبها ILLUSTRATIVE.
```

---

## 🌐 English Prompt Card

```text
Run the `rust-decision-cycle` on the scenario below and return exactly one verdict: `REJECT`, `NOT YET`, `GO (pilot)`, or `VALIDATED (scale)`.

- Workload: [What the service or product does, in 1–2 sentences]
- Business Pain: [What hurts: direct cloud cost, security/availability risk, or engineering opportunity cost? Quantify if known]
- Baseline: [Measured production metrics: p95 latency, unit cost per 1M requests/items, error rate, and target SLO]
- Bottleneck & Simpler Fixes: [Where does profiling show CPU/memory/time going? Which simpler fixes—indexing, caching, autoscaling, instance rightsizing, algorithm changes—were already tried?]
- Pilot Boundary: [What isolated slice can be tested: standalone worker/service, sidecar, WebAssembly module, CLI binary, or shared FFI core library?]
- Team & Safety Policy: [Who owns and reviews the Rust code? What is the mandatory review policy for `unsafe` and FFI boundaries?]
- Financials: [Estimated migration cost, decision horizon in months, and any recurring operational overhead]
- Verification & Rollback: [How will output parity and SLO compliance be verified? How fast can we roll back to the existing implementation?]

Rules: Mark any missing input as "Unknown". Never invent or extrapolate numbers. If numbers are hypothetical examples, label them `ILLUSTRATIVE — NOT A FORECAST`.
```
