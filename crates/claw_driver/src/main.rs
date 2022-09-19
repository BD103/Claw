use std::{env, fs, io, path::Path};

fn load_file(path: &Path) -> io::Result<String> {
    Ok(String::from_utf8_lossy(&fs::read(path)?).into_owned())
}

fn main() {
    let script = env::args().nth(1).map_or_else(
        || {
            let path = Path::new("./examples/stage1.claw");
            load_file(path).expect("Error loading file.")
        },
        |filename| {
            let path = Path::new(&filename);
            load_file(path).expect("Error loading file.")
        },
    );

    // Generate AST
    let ast = match claw::parse::parse(&script) {
        Ok(ast) => ast,
        Err(report) => {
            report
                .eprint(claw::parse::get_source(&script))
                .expect("Error writing to Stderr, please file a bug report!");
            std::process::exit(1);
        }
    };

    dbg!(ast);

    // Save to sb3 file

    #[cfg(feature = "verify")]
    {
        // Verify generated serde_json::Value
        // claw::verify::verify(project_json);
    }
}
