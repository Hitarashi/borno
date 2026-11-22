//! High-precision in-memory benchmark runner comparing string & grammar operations.

use std::{fs, time::Instant};

use convert_case::{Case, Casing};
use heck::{ToLowerCamelCase, ToSnakeCase};
use inflector::{
    cases::{
        camelcase::to_camel_case as inflector_camel, snakecase::to_snake_case as inflector_snake,
    },
    string::{
        pluralize::to_plural as inflector_plural, singularize::to_singular as inflector_singular,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Corpus {
    words_to_snake: Vec<String>,
    words_to_camel: Vec<String>,
    conforming_snake: Vec<String>,
    singular_words: Vec<String>,
    plural_words: Vec<String>,
    slug_inputs: Vec<String>,
}

#[derive(Serialize)]
struct BenchmarkMetric {
    name: String,
    total_ops: usize,
    duration_ms: f64,
    ns_per_op: f64,
    m_ops_per_sec: f64,
}

fn run_benchmark<F>(
    name: &str,
    data: &[String],
    mut operation: F,
    target_ops: usize,
) -> BenchmarkMetric
where
    F: FnMut(&str),
{
    let iterations = (target_ops / data.len()).max(1);

    // Warmup
    for _ in 0..3 {
        for item in data {
            operation(item.as_str());
        }
    }

    let start = Instant::now();
    let mut count = 0;
    for _ in 0..iterations {
        for item in data {
            operation(item.as_str());
            count += 1;
        }
    }
    let elapsed = start.elapsed();
    let duration_ns = elapsed.as_nanos() as f64;
    let duration_ms = elapsed.as_secs_f64() * 1000.0;
    let ns_per_op = duration_ns / count as f64;
    let m_ops_per_sec = (count as f64 / elapsed.as_secs_f64()) / 1_000_000.0;

    BenchmarkMetric {
        name: name.to_string(),
        total_ops: count,
        duration_ms,
        ns_per_op,
        m_ops_per_sec,
    }
}

fn main() {
    let corpus_content = fs::read_to_string("benches/data/corpus.json")
        .expect("Failed to read corpus.json. Please generate it first.");
    let corpus: Corpus =
        serde_json::from_str(&corpus_content).expect("Failed to parse corpus.json");

    let mut results: std::collections::BTreeMap<String, Vec<BenchmarkMetric>> =
        std::collections::BTreeMap::new();

    // 1. to_snake_case
    let snake_results = vec![
        run_benchmark(
            "borno",
            &corpus.words_to_snake,
            |s| {
                std::hint::black_box(borno::to_snake_case(s));
            },
            100_000,
        ),
        run_benchmark(
            "heck",
            &corpus.words_to_snake,
            |s| {
                std::hint::black_box(s.to_snake_case());
            },
            100_000,
        ),
        run_benchmark(
            "convert_case",
            &corpus.words_to_snake,
            |s| {
                std::hint::black_box(s.to_case(Case::Snake));
            },
            100_000,
        ),
        run_benchmark(
            "inflector",
            &corpus.words_to_snake,
            |s| {
                std::hint::black_box(inflector_snake(s));
            },
            100_000,
        ),
    ];
    results.insert("snake_case".to_string(), snake_results);

    // 2. to_camel_case
    let camel_results = vec![
        run_benchmark(
            "borno",
            &corpus.words_to_camel,
            |s| {
                std::hint::black_box(borno::to_camel_case(s));
            },
            100_000,
        ),
        run_benchmark(
            "heck",
            &corpus.words_to_camel,
            |s| {
                std::hint::black_box(s.to_lower_camel_case());
            },
            100_000,
        ),
        run_benchmark(
            "convert_case",
            &corpus.words_to_camel,
            |s| {
                std::hint::black_box(s.to_case(Case::Camel));
            },
            100_000,
        ),
        run_benchmark(
            "inflector",
            &corpus.words_to_camel,
            |s| {
                std::hint::black_box(inflector_camel(s));
            },
            100_000,
        ),
    ];
    results.insert("camel_case".to_string(), camel_results);

    // 3. Conforming snake_case (already conforming fast-path)
    let conforming_results = vec![
        run_benchmark(
            "borno (Cow::Borrowed)",
            &corpus.conforming_snake,
            |s| {
                std::hint::black_box(borno::to_snake_case(s));
            },
            100_000,
        ),
        run_benchmark(
            "heck",
            &corpus.conforming_snake,
            |s| {
                std::hint::black_box(s.to_snake_case());
            },
            100_000,
        ),
        run_benchmark(
            "convert_case",
            &corpus.conforming_snake,
            |s| {
                std::hint::black_box(s.to_case(Case::Snake));
            },
            100_000,
        ),
        run_benchmark(
            "inflector",
            &corpus.conforming_snake,
            |s| {
                std::hint::black_box(inflector_snake(s));
            },
            100_000,
        ),
    ];
    results.insert("conforming_snake".to_string(), conforming_results);

    // 4. Grammar: pluralize
    let plural_results = vec![
        run_benchmark(
            "borno",
            &corpus.singular_words,
            |s| {
                std::hint::black_box(borno::to_plural(s));
            },
            100_000,
        ),
        run_benchmark(
            "inflector",
            &corpus.singular_words,
            |s| {
                std::hint::black_box(inflector_plural(s));
            },
            100_000,
        ),
    ];
    results.insert("pluralize".to_string(), plural_results);

    // 5. Grammar: singularize
    let singular_results = vec![
        run_benchmark(
            "borno",
            &corpus.plural_words,
            |s| {
                std::hint::black_box(borno::to_singular(s));
            },
            100_000,
        ),
        run_benchmark(
            "inflector",
            &corpus.plural_words,
            |s| {
                std::hint::black_box(inflector_singular(s));
            },
            100_000,
        ),
    ];
    results.insert("singularize".to_string(), singular_results);

    // 6. Slugification
    let raw_slug_opts = borno::slug::SlugOptions::new().with_symbol_expansion(false);
    let slug_results = vec![
        run_benchmark(
            "borno (no symbol expansion)",
            &corpus.slug_inputs,
            |s| {
                std::hint::black_box(borno::slug::slugify_with_options(s, &raw_slug_opts));
            },
            100_000,
        ),
        run_benchmark(
            "borno",
            &corpus.slug_inputs,
            |s| {
                std::hint::black_box(borno::slugify(s));
            },
            100_000,
        ),
        run_benchmark(
            "slug",
            &corpus.slug_inputs,
            |s| {
                std::hint::black_box(slug::slugify(s));
            },
            100_000,
        ),
    ];
    results.insert("slug".to_string(), slug_results);

    // 7. Zero-allocation Predicate (borno is_snake_case vs heck s.to_snake_case() == s)
    let predicate_results = vec![
        run_benchmark(
            "borno::is_snake_case",
            &corpus.conforming_snake,
            |s| {
                std::hint::black_box(borno::case::is_snake_case(s));
            },
            100_000,
        ),
        run_benchmark(
            "heck (to_snake_case == s)",
            &corpus.conforming_snake,
            |s| {
                std::hint::black_box(s.to_snake_case() == s);
            },
            100_000,
        ),
    ];
    results.insert("predicate_validation".to_string(), predicate_results);

    let json_output = serde_json::to_string_pretty(&results).expect("Failed to serialize results");
    println!("{json_output}");
}
