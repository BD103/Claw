use std::{env, fs, io, path::Path};

fn load_file(path: &Path) -> io::Result<String> {
    Ok(String::from_utf8_lossy(&fs::read(path)?).into_owned())
}

fn main() {
    let project_json = env::args().nth(1).map_or_else(
        || {
            let path = Path::new("project.json");
            load_file(path).expect("Error loading project.json")
        },
        |filename| {
            let path = Path::new(&filename);
            load_file(path).expect("Error loading project.json")
        },
    );

    let result = claw_verify::verify_string(&project_json);

    if let Err(errors) = result {
        for error in errors {
            eprintln!("Validation Error: {:?}", error.0);
            // eprintln!("Instance Path: {}", error.1);
        }
    }
}
