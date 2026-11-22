#!/usr/bin/env python3
"""
Deterministic Benchmark Corpus Generator for borno cross-ecosystem benchmarks.
Generates canonical dataset for string casing, grammatical inflection, and slugification.
"""

import json
import os

def generate_corpus():
    base_identifiers = [
        "user_profile", "account_settings", "http_response_code", "xml_http_request",
        "oauth2_client_credentials", "api_gateway_controller", "database_connection_pool",
        "json_web_token_verifier", "order_fulfillment_status", "inventory_item_record",
        "customer_billing_address", "get_element_by_id", "parse_user_agent_header",
        "utf8_encoded_buffer", "sha256_checksum_validator", "render_virtual_dom_node",
        "calculate_tax_rate_percentage", "verify_two_factor_auth_token", "active_record_relation",
        "multi_tenant_organization_unit"
    ]

    words_to_snake = []
    words_to_camel = []
    conforming_snake = []
    for base in base_identifiers:
        # PascalCase
        pascal = "".join(part.capitalize() for part in base.split("_"))
        # camelCase
        camel = base.split("_")[0] + "".join(part.capitalize() for part in base.split("_")[1:])
        # screaming snake
        screaming = base.upper()
        # kebab
        kebab = base.replace("_", "-")
        # acronym mixes
        acronym_mixed = base.replace("http", "HTTP").replace("xml", "XML").replace("json", "JSON").replace("api", "API").replace("utf8", "UTF8")

        words_to_snake.extend([pascal, camel, screaming, kebab, acronym_mixed])
        words_to_camel.extend([base, screaming, kebab, acronym_mixed])
        conforming_snake.append(base)

    singular_words = [
        "account", "user", "order", "product", "item", "category", "tag", "post", "comment",
        "message", "notification", "address", "organization", "permission", "role", "session",
        "person", "man", "woman", "child", "ox", "mouse", "louse", "foot", "tooth", "goose",
        "die", "quiz", "matrix", "vertex", "index", "cactus", "focus", "fungus", "nucleus",
        "syllabus", "analysis", "diagnosis", "thesis", "crisis", "phenomenon", "criterion",
        "datum", "medium", "memorandum", "bacterium", "curriculum", "appendix", "basis", "axis",
        "city", "baby", "party", "fly", "supply", "box", "dish", "church", "bus", "hero",
        "potato", "tomato", "echo", "knife", "leaf", "half", "life", "wife", "shelf", "thief"
    ]

    plural_words = [
        "accounts", "users", "orders", "products", "items", "categories", "tags", "posts", "comments",
        "messages", "notifications", "addresses", "organizations", "permissions", "roles", "sessions",
        "people", "men", "women", "children", "oxen", "mice", "lice", "feet", "teeth", "geese",
        "dice", "quizzes", "matrices", "vertices", "indices", "cacti", "foci", "fungi", "nuclei",
        "syllabi", "analyses", "diagnoses", "theses", "crises", "phenomena", "criteria",
        "data", "media", "memoranda", "bacteria", "curricula", "appendices", "bases", "axes",
        "cities", "babies", "parties", "flies", "supplies", "boxes", "dishes", "churches", "buses", "heroes",
        "potatoes", "tomatoes", "echoes", "knives", "leaves", "halves", "lives", "wives", "shelves", "thieves"
    ]

    slug_inputs = [
        "Hello World! This is a 2026 Blog Post.",
        "Café & Restaurant: 100% Organic Meals",
        "Rock & Roll: Guns N' Roses Live in 東京 & Zürich",
        "Special Edition -- 50% OFF @ Spring Festival 2026!!",
        "Understanding Microservices: An In-Depth Guide (Part 1)",
        "Rust vs C++: Allocation-Conscious Performance & Safety",
        "Crème Brûlée & Piña Colada: Sweet Treats & Delights",
        "The Quick Brown Fox Jumps Over 123 Lazy Dogs & Cats"
    ]

    # Duplicate to expand corpus to ~1,000 operations per batch
    corpus = {
        "words_to_snake": words_to_snake * 10,
        "words_to_camel": words_to_camel * 10,
        "conforming_snake": conforming_snake * 50,
        "singular_words": singular_words * 15,
        "plural_words": plural_words * 15,
        "slug_inputs": slug_inputs * 100
    }

    out_dir = os.path.dirname(os.path.abspath(__file__))
    os.makedirs(out_dir, exist_ok=True)
    out_file = os.path.join(out_dir, "corpus.json")

    with open(out_file, "w", encoding="utf-8") as f:
        json.dump(corpus, f, indent=2)

    total_words = sum(len(v) for v in corpus.values())
    print(f"Generated {total_words} benchmark items in {out_file}")

if __name__ == "__main__":
    generate_corpus()
