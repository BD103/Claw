use std::{env, fs, io, path::{Path, PathBuf}};

fn load_file(path: &Path) -> io::Result<String> {
    Ok(String::from_utf8_lossy(&fs::read(path)?).into_owned())
}

fn main() -> anyhow::Result<()> {
    let (script, path) = if let Some(filename) = env::args().nth(1) {
        let path = Path::new(&filename);
        (load_file(path).expect("Error loading file."), filename)
    } else {
        let path = Path::new("./examples/parse.claw");
        (load_file(path).expect("Error loading file."), "./examples/parse.claw".to_string())
    };

    // Generate AST
    let ast = match claw::parse::parse(script.clone()) {
        Ok(ast) => ast,
        Err(report) => {
            report
                .eprint(claw::parse::get_source(script))
                .expect("Error writing to Stderr, please file a bug report!");
            std::process::exit(1);
        }
    };

    let ir = {
        use claw::ir::{create_hir, create_lir};

        create_lir(create_hir(ast))
    };

    let schema = claw::ir::create_schema(ir);
    let json = serde_json::to_string_pretty(&schema).unwrap();

    dbg!(schema);

    // Save to sb3 file
    fs::write({
        let mut pb = PathBuf::from(path);
        pb.set_extension("json");
        pb
    }, json)?;

    Ok(())
}
