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
}
