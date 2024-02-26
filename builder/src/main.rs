use {
    html_minifier::HTMLMinifier,
    std::{fs, io::Write, path::Path},
};

const CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn main() {
    let crate_root = Path::new(CRATE_ROOT);
    let root = crate_root.parent().unwrap();
    let target = crate_root.join("target");

    minify_page(&root.join("svg.html"), root, &target.join("svg.svg"));
    minify_page(
        &root.join("svg-preview.html"),
        root,
        &target.join("svg-preview.html"),
    );
    minify_page(&root.join("index.html"), root, &target.join("site.html"));
}

/// Parses a file with [`parse_page`], then minifies it with [`html_minifier`], and
/// finally writes the minimised HTML to `output`.
fn minify_page(path: &Path, root: &Path, output: &Path) {
    let html = parse_page(path, root);

    let mut minifier = HTMLMinifier::new();
    minifier.set_remove_comments(true);
    minifier.set_minify_code(false);
    minifier.digest(html.as_bytes()).unwrap();
    let minimised = std::str::from_utf8(minifier.get_html())
        .unwrap()
        .replace('\n', "");

    let mut outfile = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output)
        .unwrap();

    outfile.write_all(minimised.as_bytes()).unwrap();
}

/// Recursively reads files, and includes any files put in the `INCLUDE` macro.
fn parse_page(path: &Path, root: &Path) -> String {
    let src = fs::read_to_string(path).unwrap();
    let mut current_section = src.as_str();
    let mut base = 0;
    let mut output = String::with_capacity(src.len());

    while let Some(idx) = current_section.find("INCLUDE(") {
        let start_idx = base + idx;
        current_section = &src[start_idx..];
        let end_idx = start_idx + current_section.find(')').unwrap();
        current_section = &src[end_idx + 1..];

        let path = root.join(&src[start_idx + "INCLUDE(".len()..end_idx]);

        output += &src[base..start_idx];
        output += &parse_page(&path, path.parent().unwrap());
        base = end_idx + 1;
    }
    output += &src[base..];

    output
}
