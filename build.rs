use std::{
    env,
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

macro_rules! url {
    ($version:tt, $file:tt) => {
        concat!(
            "https://raw.githubusercontent.com/",
            "microsoft/fluentui-system-icons",
            "/refs/tags/",
            $version,
            "/fonts/",
            $file
        )
    };
}

const REGULAR_JSON_URL: &'static str = url!("1.1.341", "FluentSystemIcons-Regular.json");
const REGULAR_TTF_URL: &'static str = url!("1.1.341", "FluentSystemIcons-Regular.ttf");
const FILLED_JSON_URL: &'static str = url!("1.1.341", "FluentSystemIcons-Filled.json");
const FILLED_TTF_URL: &'static str = url!("1.1.341", "FluentSystemIcons-Filled.ttf");

pub fn main() {
    let dir = env::var("OUT_DIR").unwrap();
    let dir = Path::new(&dir);
    let file = File::create(dir.join("icons_generated.rs")).unwrap();
    let mut writer = BufWriter::new(file);

    for (name, json_url, ttf_url) in [
        ("Regular", REGULAR_JSON_URL, REGULAR_TTF_URL),
        ("Filled", FILLED_JSON_URL, FILLED_TTF_URL),
    ] {
        let mut ttf = ureq::get(ttf_url).call().unwrap().into_body().into_reader();
        let ttf_file = File::create(dir.join(format!("{name}.ttf"))).unwrap();
        let mut ttf_writer = BufWriter::new(ttf_file);
        io::copy(&mut ttf, &mut ttf_writer).unwrap();

        let json = ureq::get(json_url)
            .call()
            .unwrap()
            .body_mut()
            .read_to_string()
            .unwrap();

        writeln!(writer, "impl IconName{name} {{").unwrap();
        for line in json.lines() {
            let Some((ident, value)) = line.split_once(": ") else {
                continue;
            };

            let ident = ident.trim().trim_matches('"');
            let ident = &ident[10..ident.len() - name.len() - 1];
            let ident = ident.to_uppercase();
            let size = &ident[ident.len() - 2..];

            let value = value.trim().trim_end_matches(',').parse::<u32>().unwrap();
            let value = format!("\"\\u{{{value:x}}}\"");
            writeln!(writer, "pub const {ident}: Self = Self({value}, {size}.);").unwrap();
        }
        writeln!(writer, "}}").unwrap();
    }

    writer.flush().unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
