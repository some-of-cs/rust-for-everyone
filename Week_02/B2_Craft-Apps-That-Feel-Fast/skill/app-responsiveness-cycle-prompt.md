# App Responsiveness Cycle — Ready-to-Use Prompt (نموذج فحص سرعة واستجابة الأبليكيشن)

Use the Arabic or English prompt template below to evaluate whether a mobile, desktop, or web app should isolate a Rust core (`GO`), wait for missing measurements (`NOT YET`), or fix UI/network bottlenecks first (`REJECT`).

---

## 🇪🇬 النسخة العربية (Arabic Prompt Card)

```text
شغّل دورة app-responsiveness-cycle على السيناريو ده، واشرح أي مصطلح تقني بلغة بسيطة يفهمها المؤسس والمهندس، ورجّع لي قرار واحد بس: REJECT أو NOT YET أو GO (Bounded Pilot) أو VALIDATED (Scale).

- التطبيق: [الأبليكيشن شغال على إيه: iOS / Android / React Native / Flutter / Web / Desktop، وبيعمل إيه في جملة أو اتنين]
- شكوى العميل: [إيه اللي بيوجع العميل والبيزنس: تقييمات نجمة واحدة، تجميد وقت الـ Checkout، تقطيع في الـ Scroll، أو وقت فريقك في مشاكل البطء؟]
- قياس الأجهزة الحقيقية (Baseline): [الأرقام اللي اتقاست على موبايلات عملاء عادية مش لابتوب المبرمج: وقت الفريم مقابل 16.6 ms، نسبة التقطيع، أبطأ رد فعل P99، ووقت فتح التطبيق]
- أشعة السرعة والحلول الأبسط (Profiling): [الـ Profiler بيقول الوقت رايح فين: حجم بيانات API كبير، إعادة رسم UI، ولا حسابات CPU وتنضيف ذاكرة GC؟ وإيه الحلول الأبسط اللي جربتوها الأول زي Pagination أو Gzip أو Virtualization أو Memoization؟]
- حدود التجربة (Pilot): [إيه الجزء الحسابي الصغير اللي ممكن نعزله في Rust Core (عبر FFI أو WebAssembly أو Tauri) من غير ما نعيد بناء الـ UI كله؟]
- الأمان والفريق: [مين هيملك الكود ويراجعه؟ وهل فيه زرار أمان Feature Flag للرجوع للنسخة القديمة فوراً؟]
- الفلوس والعائد: [تكلفة التجربة التقديرية، وعايزين تغطي تكلفتها في كام شهر، وهل فيه تقدير للعملاء اللي ممكن يفضلوا أو الدعم اللي هيتوفّر؟]

قواعد: اللي مش معروف اكتب جنبه «مش معروف». وما تخترعش أرقام. لو الأرقام افتراضية، اكتب جنبها ILLUSTRATIVE — NOT A FORECAST. التزم بترقيم الخطوات من Step 1 لحد Step 8.
```

---

## 🌍 English Prompt Card

```text
Run the `app-responsiveness-cycle` on the scenario below, explain technical terms plainly so both founders and engineers can follow, and return exactly one verdict: `REJECT`, `NOT YET`, `GO (Bounded Pilot)`, or `VALIDATED (Scale)`.

- App & Platforms: [Mobile / Desktop / Web stack (e.g., React Native, Flutter, Swift/Kotlin, Electron/Web) and what the app does in 1–2 sentences]
- User & Business Pain: [1-star reviews, checkout abandonment, scrolling jank, freezes, or engineering hours lost to lag bugs]
- Real-Device Baseline: [Measured on mid/low-tier user devices (not only a dev laptop): frame time vs. 16.6 ms / 8.3 ms, dropped frame %, P99 input latency, and cold startup time]
- Profiling & Simpler Fixes Tried: [Where does the profiler show main-thread time going? Which simpler fixes—pagination, Gzip/Brotli, list virtualization, component memoization, image compression—were already tried?]
- Bounded Pilot Scope: [Which isolated CPU/parsing/sync module could move into a Rust core via FFI / UniFFI, WebAssembly, or Tauri while keeping the existing UI?]
- Safety, Rollback & Team: [Who owns and reviews the Rust/FFI code? Is there an instant Feature-Flag rollback to the legacy path?]
- Financials: [Estimated pilot build cost, target payback horizon in months, and estimated monthly retained revenue + support savings]

Rules: Mark any missing input as "Unknown". Never invent or extrapolate numbers. Label hypothetical examples `ILLUSTRATIVE — NOT A FORECAST`. Number every step explicitly from Step 1 to Step 8.
```
