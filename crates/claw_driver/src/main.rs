use std::{env, fs, io, path::Path};

fn load_file(path: &Path) -> io::Result<String> {
    Ok(String::from_utf8_lossy(&fs::read(path)?).into_owned())
}

fn main() -> anyhow::Result<()> {
    let script = if let Some(filename) = env::args().nth(1) {
        let path = Path::new(&filename);
        load_file(path).expect("Error loading file.")
    } else {
        let path = Path::new("./examples/stage1.claw");
        load_file(path).expect("Error loading file.")
    };

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

    Ok(())
}
