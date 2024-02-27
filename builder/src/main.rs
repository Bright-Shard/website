use {
    html_minifier::HTMLMinifier,
    std::{fs, io::Write, path::Path},
};

const CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn main() {
    let crate_root = Path::new(CRATE_ROOT);
    let root = crate_root.parent().unwrap();
    let out_dir = crate_root.join("target").join("site");

    if !out_dir.exists() {
        fs::create_dir(&out_dir).unwrap();
    }

    minify(
        &root.join("index.svg"),
        root,
        &out_dir.join("site.svg"),
        true,
    );
    minify(
        &root.join("svg-preview.html"),
        root,
        &out_dir.join("svg-preview.html"),
        true,
    );
    minify(
        &root.join("index.html"),
        root,
        &out_dir.join("index.html"),
        true,
    );
    minify(
        &root.join("css").join("main.css"),
        &root.join("css"),
        &out_dir.join("style.css"),
        false,
    );

    let fonts_out = out_dir.join("fonts");
    if !fonts_out.exists() {
        fs::create_dir(&fonts_out).unwrap();
    }
    for font_file in root.join("fonts").read_dir().unwrap() {
        let font_file = font_file.unwrap();
        fs::copy(font_file.path(), fonts_out.join(font_file.file_name())).unwrap();
    }
}

/// Parses a file with [`parse_page`], then minifies it with [`html_minifier`], and
/// finally writes the minimised HTML to `output`.
fn minify(path: &Path, root: &Path, output: &Path, minify: bool) {
    let html = handle_macros(path, root);

    let minimised = if minify {
        let mut minifier = HTMLMinifier::new();
        minifier.set_remove_comments(true);
        minifier.set_minify_code(false);
        minifier.digest(html.as_bytes()).unwrap();
        std::str::from_utf8(minifier.get_html())
            .unwrap()
            .replace('\n', "")
    } else {
        html
    };

    let mut outfile = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output)
        .unwrap();

    outfile.write_all(minimised.as_bytes()).unwrap();
}

/// Recursively reads files, and includes any files put in the `INCLUDE` macro.
fn handle_macros(path: &Path, root: &Path) -> String {
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
        output += &handle_macros(&path, path.parent().unwrap());
        base = end_idx + 1;
    }
    output += &src[base..];

    output
}
