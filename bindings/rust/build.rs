fn compile(name: &str, src_dir: &std::path::Path) {
    let mut c_config = cc::Build::new();
    c_config.std("c11").include(src_dir);

    #[cfg(target_env = "msvc")]
    c_config.flag("-utf-8");

    let parser_path = src_dir.join("parser.c");
    c_config.file(&parser_path);
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());

    let scanner_path = src_dir.join("scanner.c");
    if scanner_path.exists() {
        c_config.file(&scanner_path);
        println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());
    }

    c_config.compile(name);
}

fn main() {
    // The resource-file grammar and the bare-message grammar used for injections.
    compile("tree-sitter-messageformat", std::path::Path::new("src"));
    compile("tree-sitter-messageformat-message", std::path::Path::new("message/src"));
}
