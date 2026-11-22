#![cfg(all(feature = "builder", feature = "full"))]

use borno::{StrBuilder, slug::SlugOptions, utils::TruncateOptions};

#[test]
fn test_builder_multi_step_pipelines() {
    let table = StrBuilder::new("AdminUserProfile")
        .snake_case()
        .pluralize()
        .into_string();
    assert_eq!(table, "admin_user_profiles");

    let model = StrBuilder::new("admin_user_profiles")
        .singularize()
        .pascal_case()
        .into_string();
    assert_eq!(model, "AdminUserProfile");

    let fk = StrBuilder::new("UserProfile").foreign_key().into_string();
    assert_eq!(fk, "user_profile_id");

    let slug = StrBuilder::new("  Rust & Borno Library: Fast!  ")
        .slugify()
        .into_string();
    assert_eq!(slug, "rust-and-borno-library-fast");

    let guarded_url = StrBuilder::new("https://example.com/api")
        .finish("/")
        .wrap("<", ">")
        .into_string();
    assert_eq!(guarded_url, "<https://example.com/api/>");
}

#[test]
fn test_builder_all_methods() {
    let screaming = StrBuilder::from("user_id").screaming_snake_case();
    assert_eq!(screaming.as_str(), "USER_ID");
    assert_eq!(format!("{screaming}"), "USER_ID");
    assert_eq!(screaming.as_ref(), "USER_ID");
    assert_eq!(&*screaming, "USER_ID");

    let kebab = StrBuilder::new("user_id").kebab_case().into_string();
    assert_eq!(kebab, "user-id");

    let train = StrBuilder::new("user_id").train_case().into_string();
    assert_eq!(train, "User-Id");

    let title = StrBuilder::new("user_id").title_case().into_string();
    assert_eq!(title, "User Id");

    let sentence = StrBuilder::new("user_id").sentence_case().into_string();
    assert_eq!(sentence, "User id");

    let classify = StrBuilder::new("users").classify().into_string();
    assert_eq!(classify, "User");

    let count_sing = StrBuilder::new("boxes")
        .pluralize_with_count(1)
        .into_string();
    assert_eq!(count_sing, "box");

    let count_plur = StrBuilder::new("box").pluralize_with_count(2).into_string();
    assert_eq!(count_plur, "boxes");

    let slug_opts = StrBuilder::new("Custom Title")
        .slugify_with_options(&SlugOptions::new().with_delimiter('_'))
        .into_string();
    assert_eq!(slug_opts, "custom_title");

    let trunc = StrBuilder::new("Very long string that needs truncation")
        .truncate(12)
        .into_string();
    assert_eq!(trunc, "Very long...");

    let trunc_opts = StrBuilder::new("Another long string")
        .truncate_with_options(15, &TruncateOptions::new().with_suffix("##"))
        .into_string();
    assert_eq!(trunc_opts, "Another long ##");

    let words = StrBuilder::new("One two three four five")
        .words(3, "...")
        .into_string();
    assert_eq!(words, "One two three...");

    let masked = StrBuilder::new("secret-value")
        .mask('*', 2, Some(4))
        .into_string();
    assert_eq!(masked, "se****-value");

    let started = StrBuilder::new("path").start("/").into_string();
    assert_eq!(started, "/path");

    let dot = StrBuilder::new("user_account_id").dot_case().into_string();
    assert_eq!(dot, "user.account.id");

    let hl = StrBuilder::new("user_profile_id").headline().into_string();
    assert_eq!(hl, "User Profile Id");

    let hum = StrBuilder::new("author_id").humanize().into_string();
    assert_eq!(hum, "Author");

    let squished = StrBuilder::new("  lots   of   spaces  \n\t ")
        .squish()
        .into_string();
    assert_eq!(squished, "lots of spaces");

    let norm_slash = StrBuilder::new("path\\to\\sub\\file.txt")
        .normalize_slashes()
        .into_string();
    assert_eq!(norm_slash, "path/to/sub/file.txt");

    let escaped = StrBuilder::new("<div>Hello & Welcome</div>")
        .html_escape()
        .into_string();
    assert_eq!(escaped, "&lt;div&gt;Hello &amp; Welcome&lt;/div&gt;");

    let unescaped = StrBuilder::new("&lt;b&gt;bold&lt;/b&gt;")
        .html_unescape()
        .into_string();
    assert_eq!(unescaped, "<b>bold</b>");

    let wrapped = StrBuilder::new("The quick brown fox jumps over the lazy dog")
        .word_wrap(15)
        .into_string();
    assert_eq!(wrapped, "The quick brown\nfox jumps over\nthe lazy dog");

    let interp = StrBuilder::new("Welcome to {{ name }}!")
        .interpolate(&[("name", "Borno")])
        .into_string();
    assert_eq!(interp, "Welcome to Borno!");

    let interp_p = StrBuilder::new("Host: :host Port: :port")
        .interpolate(&[("host", "localhost"), ("port", "8080")])
        .into_string();
    assert_eq!(interp_p, "Host: localhost Port: 8080");

    let trimmed = StrBuilder::new("  hello world  ").trim().build();
    assert_eq!(trimmed, "hello world");

    let from_str: StrBuilder = "test".into();
    assert_eq!(from_str.into_string(), "test");

    let from_string: StrBuilder = String::from("test").into();
    assert_eq!(from_string.into_string(), "test");

    let no_c = StrBuilder::new("UserProfile").no_case().into_string();
    assert_eq!(no_c, "user profile");

    let cap_c = StrBuilder::new("user_profile").capital_case().into_string();
    assert_eq!(cap_c, "User Profile");

    let cap_f = StrBuilder::new("hello").capitalize_first().into_string();
    assert_eq!(cap_f, "Hello");

    let uncap_f = StrBuilder::new("Hello").uncapitalize_first().into_string();
    assert_eq!(uncap_f, "hello");

    let studly = StrBuilder::new("user_id").studly_case().into_string();
    assert_eq!(studly, "UserId");

    let cls = StrBuilder::new("user_id").class_case().into_string();
    assert_eq!(cls, "UserId");

    let tbl = StrBuilder::new("UserProfile").table_case().into_string();
    assert_eq!(tbl, "user_profiles");

    let to_pl = StrBuilder::new("user").to_plural().into_string();
    assert_eq!(to_pl, "users");

    let to_sing = StrBuilder::new("users").to_singular().into_string();
    assert_eq!(to_sing, "user");
}

#[test]
fn test_builder_flow_control_and_affix() {
    fn append_bang(b: StrBuilder) -> StrBuilder {
        b.append("!")
    }
    let b_t = StrBuilder::new("button")
        .when(true, append_bang)
        .into_string();
    assert_eq!(b_t, "button!");

    let b_f = StrBuilder::new("button")
        .when(false, append_bang)
        .into_string();
    assert_eq!(b_f, "button");

    let empty_t = StrBuilder::new("").when_empty(append_bang).into_string();
    assert_eq!(empty_t, "!");

    let empty_f = StrBuilder::new("val").when_empty(append_bang).into_string();
    assert_eq!(empty_f, "val");

    let not_empty_t = StrBuilder::new("content")
        .when_not_empty(append_bang)
        .into_string();
    assert_eq!(not_empty_t, "content!");

    let not_empty_f = StrBuilder::new("")
        .when_not_empty(append_bang)
        .into_string();
    assert_eq!(not_empty_f, "");

    let mut tapped = false;
    let tapped_val = StrBuilder::new("val").tap(|_| tapped = true).into_string();
    assert!(tapped);
    assert_eq!(tapped_val, "val");

    let piped = StrBuilder::new("hello").pipe(|builder| builder.into_string().to_uppercase());
    assert_eq!(piped, "HELLO");

    let inspect_b = StrBuilder::new("hello world");
    assert!(!inspect_b.is_empty());
    assert!(inspect_b.is_not_empty());
    assert!(inspect_b.starts_with_any(&["hi", "hello"]));
    assert!(inspect_b.ends_with_any(&["world", "earth"]));

    let affixes = StrBuilder::new("User")
        .suffix("Controller")
        .prefix("Api")
        .ext("rs")
        .into_string();
    assert_eq!(affixes, "ApiUserController.rs");

    let stripped = StrBuilder::new("ApiUserController.rs")
        .remove_prefix("Api")
        .remove_suffix("Controller.rs")
        .into_string();
    assert_eq!(stripped, "User");

    let ext_stripped = StrBuilder::new("archive.tar.gz")
        .remove_extension()
        .into_string();
    assert_eq!(ext_stripped, "archive.tar");

    let replaced = StrBuilder::new("foo bar foo baz")
        .replace_first("foo", "qux")
        .replace_last("foo", "qux")
        .remove("qux ")
        .deduplicate('o')
        .into_string();
    assert_eq!(replaced, "bar baz");
}

#[test]
fn test_builder_cow_zero_allocation_pipeline() {
    use std::borrow::Cow;

    use borno::{Case, StrBuilder};

    let original = "user_account_id";
    let builder = StrBuilder::new(original).trim().to_case(Case::Snake);
    assert_eq!(builder.into_cow(), Cow::Borrowed("user_account_id"));

    let sliced_builder = StrBuilder::new(original).before("_id");
    assert_eq!(sliced_builder.into_cow(), Cow::Borrowed("user_account"));

    let modified = StrBuilder::new(original)
        .to_case(Case::Camel)
        .finish("!")
        .into_cow();
    assert_eq!(modified, Cow::Owned::<str>("userAccountId!".to_string()));
}

#[test]
fn test_builder_coverage_gaps() {
    // Branches where transformation changes the string
    assert_eq!(
        StrBuilder::new("the old man and the sea")
            .title_case_apa()
            .into_string(),
        "The Old Man and the Sea"
    );
    assert_eq!(
        StrBuilder::new("hello").capitalize_first().into_string(),
        "Hello"
    );
    assert_eq!(
        StrBuilder::new(String::from("App::Models::User"))
            .demodulize()
            .into_string(),
        "User"
    );
    assert_eq!(
        StrBuilder::new(String::from("App::Models::User"))
            .deconstantize()
            .into_string(),
        "App::Models"
    );
    assert_eq!(StrBuilder::new("user").pluralize().into_string(), "users");
    assert_eq!(
        StrBuilder::new("user")
            .pluralize_with_count(2)
            .into_string(),
        "users"
    );
    assert_eq!(StrBuilder::new("path").start("/").into_string(), "/path");
    assert_eq!(StrBuilder::new("path").finish("/").into_string(), "path/");
    assert_eq!(
        StrBuilder::new("path").ensure_wrap("<", ">").into_string(),
        "<path>"
    );
    assert_eq!(
        StrBuilder::new(String::from("user_profile_id"))
            .after("_")
            .into_string(),
        "profile_id"
    );
    assert_eq!(
        StrBuilder::new(String::from("user_profile_id"))
            .after_last("_")
            .into_string(),
        "id"
    );
    assert_eq!(
        StrBuilder::new(String::from("<div>content</div>"))
            .between("<div>", "</div>")
            .into_string(),
        "content"
    );
    assert_eq!(
        StrBuilder::new(String::from("<div>first</div><div>second</div>"))
            .between_first("<div>", "</div>")
            .into_string(),
        "first"
    );
    assert_eq!(
        StrBuilder::new("  hello   world  ").squish().into_string(),
        "hello world"
    );
    assert_eq!(
        StrBuilder::new(r"path\to\file")
            .normalize_slashes()
            .into_string(),
        "path/to/file"
    );
    assert_eq!(
        StrBuilder::new("<script>alert('xss')</script>")
            .html_escape()
            .into_string(),
        "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;"
    );
    assert_eq!(
        StrBuilder::new("&lt;b&gt;bold&lt;/b&gt;")
            .html_unescape()
            .into_string(),
        "<b>bold</b>"
    );

    // Branches where transformation does NOT change the string (zero-allocation / else branches)
    assert_eq!(
        StrBuilder::new("The Old Man and the Sea")
            .title_case_apa()
            .into_string(),
        "The Old Man and the Sea"
    );
    assert_eq!(
        StrBuilder::new("Hello").capitalize_first().into_string(),
        "Hello"
    );
    assert_eq!(
        StrBuilder::new(String::from("User"))
            .demodulize()
            .into_string(),
        "User"
    );
    assert_eq!(
        StrBuilder::new(String::from(""))
            .deconstantize()
            .into_string(),
        ""
    );
    assert_eq!(StrBuilder::new("fish").pluralize().into_string(), "fish");
    assert_eq!(
        StrBuilder::new("fish")
            .pluralize_with_count(2)
            .into_string(),
        "fish"
    );
    assert_eq!(StrBuilder::new("/path").start("/").into_string(), "/path");
    assert_eq!(StrBuilder::new("path/").finish("/").into_string(), "path/");
    assert_eq!(
        StrBuilder::new("<path>")
            .ensure_wrap("<", ">")
            .into_string(),
        "<path>"
    );
    assert_eq!(
        StrBuilder::new(String::from("user"))
            .after("")
            .into_string(),
        "user"
    );
    assert_eq!(
        StrBuilder::new(String::from("user"))
            .after_last("")
            .into_string(),
        "user"
    );
    assert_eq!(
        StrBuilder::new(String::from(""))
            .between("<a>", "</a>")
            .into_string(),
        ""
    );
    assert_eq!(
        StrBuilder::new(String::from(""))
            .between_first("<a>", "</a>")
            .into_string(),
        ""
    );
    assert_eq!(
        StrBuilder::new("hello world").squish().into_string(),
        "hello world"
    );
    assert_eq!(
        StrBuilder::new("path/to/file")
            .normalize_slashes()
            .into_string(),
        "path/to/file"
    );
    assert_eq!(
        StrBuilder::new("hello").html_escape().into_string(),
        "hello"
    );
    assert_eq!(
        StrBuilder::new("hello").html_unescape().into_string(),
        "hello"
    );
}
