# Cross-Ecosystem Performance Benchmark Report

Benchmarked on canonical corpus across **Rust (borno, heck, convert_case, inflector, slug)**, **Node.js (V8)**, and **Bun (JavaScriptCore)**.


### to_snake_case (Standard & Acronym Identifiers)

| Tool / Runtime | Implementation | Latency (ns/op) | Throughput (M ops/s) | Speedup vs Reference |
| :--- | :--- | :--- | :--- | :--- |
| Rust (borno) | `Rust (borno)` | **124.9 ns** | **8.00 M** | 1.00x (Fastest) |
| Rust (inflector) | `Rust (inflector)` | **349.4 ns** | **2.86 M** | 2.80x |
| Node.js (es-toolkit) | `Node.js (es-toolkit)` | **619.1 ns** | **1.62 M** | 4.95x |
| Node.js (lodash) | `Node.js (lodash)` | **725.2 ns** | **1.38 M** | 5.80x |
| Rust (heck) | `Rust (heck)` | **729.4 ns** | **1.37 M** | 5.84x |
| Bun (es-toolkit) | `Bun (es-toolkit)` | **1184.5 ns** | **0.84 M** | 9.48x |
| Bun (@poppinss/string) | `Bun (@poppinss/string)` | **1573.5 ns** | **0.64 M** | 12.59x |
| Bun (lodash) | `Bun (lodash)` | **1670.0 ns** | **0.60 M** | 13.37x |
| Rust (convert_case) | `Rust (convert_case)` | **1990.1 ns** | **0.50 M** | 15.93x |
| Node.js (@poppinss/string) | `Node.js (@poppinss/string)` | **2703.1 ns** | **0.37 M** | 21.63x |

### to_camel_case (Standard Identifiers)

| Tool / Runtime | Implementation | Latency (ns/op) | Throughput (M ops/s) | Speedup vs Reference |
| :--- | :--- | :--- | :--- | :--- |
| Rust (borno) | `Rust (borno)` | **123.1 ns** | **8.12 M** | 1.00x (Fastest) |
| Rust (inflector) | `Rust (inflector)` | **141.9 ns** | **7.05 M** | 1.15x |
| Node.js (es-toolkit) | `Node.js (es-toolkit)` | **740.5 ns** | **1.35 M** | 6.01x |
| Rust (heck) | `Rust (heck)` | **856.6 ns** | **1.17 M** | 6.96x |
| Node.js (lodash) | `Node.js (lodash)` | **989.7 ns** | **1.01 M** | 8.04x |
| Bun (es-toolkit) | `Bun (es-toolkit)` | **2204.1 ns** | **0.45 M** | 17.90x |
| Rust (convert_case) | `Rust (convert_case)` | **2244.3 ns** | **0.45 M** | 18.23x |
| Node.js (@poppinss/string) | `Node.js (@poppinss/string)` | **2851.6 ns** | **0.35 M** | 23.16x |
| Bun (lodash) | `Bun (lodash)` | **4241.7 ns** | **0.24 M** | 34.46x |
| Bun (@poppinss/string) | `Bun (@poppinss/string)` | **6033.3 ns** | **0.17 M** | 49.01x |

### Zero-Allocation Conforming String Fast-Path

| Tool / Runtime | Implementation | Latency (ns/op) | Throughput (M ops/s) | Speedup vs Reference |
| :--- | :--- | :--- | :--- | :--- |
| Rust (borno (Cow::Borrowed)) | `Rust (borno (Cow::Borrowed))` | **32.0 ns** | **31.21 M** | 1.00x (Fastest) |
| Rust (inflector) | `Rust (inflector)` | **151.7 ns** | **6.59 M** | 4.73x |
| Node.js (es-toolkit) | `Node.js (es-toolkit)` | **471.2 ns** | **2.12 M** | 14.71x |
| Node.js (lodash) | `Node.js (lodash)` | **684.3 ns** | **1.46 M** | 21.35x |
| Rust (heck) | `Rust (heck)` | **783.8 ns** | **1.28 M** | 24.46x |
| Bun (es-toolkit) | `Bun (es-toolkit)` | **962.3 ns** | **1.04 M** | 30.03x |
| Node.js (@poppinss/string) | `Node.js (@poppinss/string)` | **1723.2 ns** | **0.58 M** | 53.78x |
| Rust (convert_case) | `Rust (convert_case)` | **2535.3 ns** | **0.39 M** | 79.12x |
| Bun (lodash) | `Bun (lodash)` | **4252.8 ns** | **0.24 M** | 132.72x |
| Bun (@poppinss/string) | `Bun (@poppinss/string)` | **4666.3 ns** | **0.21 M** | 145.62x |

### Predicate Validation (is_snake_case vs to_snake_case == s)

| Tool / Runtime | Implementation | Latency (ns/op) | Throughput (M ops/s) | Speedup vs Reference |
| :--- | :--- | :--- | :--- | :--- |
| Rust (borno::is_snake_case) | `Rust (borno::is_snake_case)` | **29.4 ns** | **34.04 M** | 1.00x (Fastest) |
| Rust (heck (to_snake_case == s)) | `Rust (heck (to_snake_case == s))` | **645.1 ns** | **1.55 M** | 21.96x |

### English Pluralization (Singular to Plural)

| Tool / Runtime | Implementation | Latency (ns/op) | Throughput (M ops/s) | Speedup vs Reference |
| :--- | :--- | :--- | :--- | :--- |
| Rust (borno) | `Rust (borno)` | **229.9 ns** | **4.35 M** | 1.00x (Fastest) |
| Rust (inflector) | `Rust (inflector)` | **1451.6 ns** | **0.69 M** | 6.31x |
| Bun (pluralize) | `Bun (pluralize)` | **1605.7 ns** | **0.62 M** | 6.98x |
| Node.js (pluralize) | `Node.js (pluralize)` | **1699.9 ns** | **0.59 M** | 7.39x |
| Node.js (@poppinss/string) | `Node.js (@poppinss/string)` | **1731.9 ns** | **0.58 M** | 7.53x |
| Bun (@poppinss/string) | `Bun (@poppinss/string)` | **4679.0 ns** | **0.21 M** | 20.35x |

### English Singularization (Plural to Singular)

| Tool / Runtime | Implementation | Latency (ns/op) | Throughput (M ops/s) | Speedup vs Reference |
| :--- | :--- | :--- | :--- | :--- |
| Rust (borno) | `Rust (borno)` | **232.6 ns** | **4.30 M** | 1.00x (Fastest) |
| Bun (@poppinss/string) | `Bun (@poppinss/string)` | **1484.0 ns** | **0.67 M** | 6.38x |
| Node.js (@poppinss/string) | `Node.js (@poppinss/string)` | **1504.4 ns** | **0.66 M** | 6.47x |
| Node.js (pluralize) | `Node.js (pluralize)` | **1606.8 ns** | **0.62 M** | 6.91x |
| Rust (inflector) | `Rust (inflector)` | **1822.0 ns** | **0.55 M** | 7.83x |
| Bun (pluralize) | `Bun (pluralize)` | **4552.0 ns** | **0.22 M** | 19.57x |

### URL Slugification & Transliteration

| Tool / Runtime | Implementation | Latency (ns/op) | Throughput (M ops/s) | Speedup vs Reference |
| :--- | :--- | :--- | :--- | :--- |
| Rust (borno (no symbol expansion)) | `Rust (borno (no symbol expansion))` | **81.8 ns** | **12.22 M** | 1.00x (Fastest) |
| Rust (slug) | `Rust (slug)` | **103.0 ns** | **9.70 M** | 1.26x |
| Rust (borno) | `Rust (borno)` | **104.1 ns** | **9.60 M** | 1.27x |
| Node.js (es-toolkit (kebabCase)) | `Node.js (es-toolkit (kebabCase))` | **1296.2 ns** | **0.77 M** | 15.84x |
| Node.js (@poppinss/string) | `Node.js (@poppinss/string)` | **6267.1 ns** | **0.16 M** | 76.61x |
| Bun (es-toolkit (kebabCase)) | `Bun (es-toolkit (kebabCase))` | **6505.3 ns** | **0.15 M** | 79.52x |
| Bun (@poppinss/string) | `Bun (@poppinss/string)` | **27071.0 ns** | **0.04 M** | 330.90x |

---

## Methodology & Architectural Highlights

1. **Deterministic Test Corpus**:
   - All tests across Rust, Node.js (V8), and Bun (JavaScriptCore) evaluate the identical canonical corpus generated deterministically by `benches/data/generate_corpus.py`.
   - The corpus includes realistic real-world acronym mixes (`HTTPResponseCode`, `XMLHTTPRequest`, `ActiveRecord`), standard identifiers, irregular plurals (`oxen`, `syllabi`, `phenomena`), and multilingual strings with Latin diacritics and CJK characters.

2. **Zero-Allocation Fast Paths**:
   - `borno` leverages `Cow<'_, str>` for casing transformations. When inputs already conform to target casing (e.g., `user_profile` passed to `to_snake_case`), `borno` completes in **~27 ns** with zero heap allocations, yielding an immediate **4.7x speedup over inflector** and **23x speedup over heck**.

3. **Pure Byte-Scanning Algorithms**:
   - ASCII word boundary detection (`for_each_word_ascii`) eliminates intermediate vector allocations and intermediate string assembly.
   - Output buffers stream directly into pre-allocated memory via sound, bounds-safe ASCII byte operations.

4. **URL Slugification & Grammatical Expansion**:
   - By default, `borno::slugify` executes English grammatical symbol expansion (`&` -> `"and"`, `@` -> `"at"`), preserving semantic intent in URLs.
   - When configured for raw slugification matching the baseline feature set of alternative tools (`SlugOptions::new().with_symbol_expansion(false)`), `borno` outperforms all competing crates and JavaScript/TypeScript runtimes at **89.0 ns/op (11.24 M ops/s)**.
