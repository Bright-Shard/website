use {
    base64::{engine::general_purpose::STANDARD as BASE64_ENCODER, Engine as _},
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

    compile("index.svg", root, &out_dir);
    compile("index.html", root, &out_dir);
    compile("svg-preview.html", root, &out_dir);
    compile("main.css", &root.join("css"), &out_dir);

    let fonts_out = out_dir.join("fonts");
    if !fonts_out.exists() {
        fs::create_dir(&fonts_out).unwrap();
    }
    for font_file in root.join("fonts").read_dir().unwrap() {
        let font_file = font_file.unwrap();
        fs::copy(font_file.path(), fonts_out.join(font_file.file_name())).unwrap();
    }
}

fn compile(file_name: &str, root: &Path, out_dir: &Path) {
    let input_file = root.join(file_name);
    let output_file = out_dir.join(file_name);

    let compiled = parse_file(&input_file, root);

    let mut outfile = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_file)
        .unwrap();
    outfile.write_all(compiled.as_bytes()).unwrap();
}

/// Reads a file, handles its macros, then returns a minimised version of it.
fn parse_file(file: &Path, root: &Path) -> String {
    println!("Parsing `{}`", file.display());
    let src = fs::read_to_string(file).unwrap();
    let mut current_section = src.as_str();
    let mut last_handled_idx = 0;
    let mut output = String::with_capacity(src.len());

    while let Some(macro_) = find_macro(current_section) {
        output += &src[last_handled_idx..last_handled_idx + macro_.start];
        output += &macro_.run(root);
        last_handled_idx = last_handled_idx + macro_.end + 1;
        current_section = &src[last_handled_idx..];
    }
    output += &src[last_handled_idx..];

    match file.extension().unwrap().to_str().unwrap() {
        "html" | "svg" => {
            let mut minifier = HTMLMinifier::new();
            minifier.set_remove_comments(true);
            minifier.set_minify_code(false);
            minifier.digest(&output).unwrap();
            std::str::from_utf8(minifier.get_html())
                .unwrap()
                .to_string()
        }
        "css" => minifier::css::minify(&output).unwrap().to_string(),
        other => panic!("Unknown file extension `{other}`."),
    }
}

/// Attempts to find a single macro in `src`.
fn find_macro(src: &str) -> Option<Macro> {
    if let Some(macro_idx) = src.find("#!") {
        let macro_src = &src[macro_idx..];
        let args_start_idx = macro_idx + macro_src.find('(').unwrap();
        let mut args_end_idx = 0;
        let mut num_parentheses = 1;
        for (idx, letter) in src[args_start_idx + 1..].as_bytes().iter().enumerate() {
            match *letter {
                b'(' => num_parentheses += 1,
                b')' => {
                    num_parentheses -= 1;
                    if num_parentheses == 0 {
                        args_end_idx = idx;
                        break;
                    }
                }
                _ => {}
            }
        }
        if num_parentheses != 0 {
            panic!("Unclosed parenthesis");
        }
        args_end_idx += args_start_idx + 1;

        let macro_name = &src[macro_idx + 2..args_start_idx];
        let macro_args = &src[args_start_idx + 1..args_end_idx];

        Some(Macro {
            name: macro_name,
            args: macro_args,
            start: macro_idx,
            end: args_end_idx,
        })
    } else {
        None
    }
}

struct Macro<'a> {
    name: &'a str,
    args: &'a str,
    start: usize,
    end: usize,
}
impl Macro<'_> {
    pub fn run(&self, root: &Path) -> String {
        match self.name {
            "INCLUDE" => {
                let path = root.join(self.args);
                let root = path.parent().unwrap();
                parse_file(&path, root)
            }
            "BASE64" => match find_macro(self.args) {
                Some(macro_) => BASE64_ENCODER.encode(macro_.run(root)),
                None => BASE64_ENCODER.encode(self.args),
            },
            "INCLUDE_BASE64" => {
                let path = root.join(self.args);
                let input = fs::read(path).unwrap();
                BASE64_ENCODER.encode(input)
            }
            other => panic!("Unknown macro `{other}`."),
        }
    }
}
