import fs from 'node:fs';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';
import { snakeCase, camelCase, kebabCase } from 'es-toolkit/string';
import _ from 'lodash';
import string from '@poppinss/string';
import pluralize from 'pluralize';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const corpusPath = path.resolve(__dirname, '../data/corpus.json');
const corpus = JSON.parse(fs.readFileSync(corpusPath, 'utf8'));

function benchmark(name, data, fn, targetOps = 100000) {
    const iterations = Math.max(1, Math.ceil(targetOps / data.length));
    
    // Warmup
    for (let i = 0; i < 3; i++) {
        for (let j = 0; j < data.length; j++) {
            fn(data[j]);
        }
    }

    const start = process.hrtime.bigint();
    let count = 0;
    for (let i = 0; i < iterations; i++) {
        for (let j = 0; j < data.length; j++) {
            fn(data[j]);
            count++;
        }
    }
    const end = process.hrtime.bigint();
    const durationNs = Number(end - start);
    const durationSec = durationNs / 1e9;
    const nsPerOp = durationNs / count;
    const mOpsPerSec = (count / durationSec) / 1e6;

    return {
        name,
        total_ops: count,
        duration_ms: durationNs / 1e6,
        ns_per_op: nsPerOp,
        m_ops_per_sec: mOpsPerSec
    };
}

const results = {};

// 1. to_snake_case
results['snake_case'] = [
    benchmark('es-toolkit', corpus.words_to_snake, (w) => snakeCase(w)),
    benchmark('@poppinss/string', corpus.words_to_snake, (w) => string.snakeCase(w)),
    benchmark('lodash', corpus.words_to_snake, (w) => _.snakeCase(w))
];

// 2. to_camel_case
results['camel_case'] = [
    benchmark('es-toolkit', corpus.words_to_camel, (w) => camelCase(w)),
    benchmark('@poppinss/string', corpus.words_to_camel, (w) => string.camelCase(w)),
    benchmark('lodash', corpus.words_to_camel, (w) => _.camelCase(w))
];

// 3. Conforming snake_case (already formatted)
results['conforming_snake'] = [
    benchmark('es-toolkit', corpus.conforming_snake, (w) => snakeCase(w)),
    benchmark('@poppinss/string', corpus.conforming_snake, (w) => string.snakeCase(w)),
    benchmark('lodash', corpus.conforming_snake, (w) => _.snakeCase(w))
];

// 4. Grammar: pluralize
results['pluralize'] = [
    benchmark('pluralize', corpus.singular_words, (w) => pluralize(w)),
    benchmark('@poppinss/string', corpus.singular_words, (w) => string.pluralize(w))
];

// 5. Grammar: singularize
results['singularize'] = [
    benchmark('pluralize', corpus.plural_words, (w) => pluralize.singular(w)),
    benchmark('@poppinss/string', corpus.plural_words, (w) => string.singular(w))
];

// 6. Slugification
results['slug'] = [
    benchmark('@poppinss/string', corpus.slug_inputs, (w) => string.slug(w)),
    benchmark('es-toolkit (kebabCase)', corpus.slug_inputs, (w) => kebabCase(w))
];

console.log(JSON.stringify(results, null, 2));
