use anyhow::anyhow;
use jsonschema::{JSONSchema, SchemaResolver, SchemaResolverError};
use std::{env, fs, io, path::Path, sync::Arc};
use url::Url;

const SB3_SCHEMA: &str = include_str!("sb3_schema.json");
const SB3_DEFINITIONS: &str = include_str!("sb3_definitions.json");

struct Sb3Resolver {}

impl SchemaResolver for Sb3Resolver {
    fn resolve(
        &self,
        _root_schema: &serde_json::Value,
        url: &Url,
        _original_reference: &str,
    ) -> Result<Arc<serde_json::Value>, SchemaResolverError> {
        match (url.scheme(), url.path()) {
            ("json-schema", "/sb3_definitions.json") => Ok(Arc::new(
                serde_json::from_str(SB3_DEFINITIONS)
                    .expect("Error deserializing JSON definitions."),
            )),
            (_, _) => Err(anyhow!("Could not load schema.")),
        }
    }
}

fn load_file(path: &Path) -> io::Result<String> {
    Ok(String::from_utf8_lossy(&fs::read(path)?).into_owned())
}

fn main() {
    let project_json = if let Some(filename) = env::args().nth(1) {
        let path = Path::new(&filename);
        load_file(path).expect("Error loading project.json.")
    } else {
        let path = Path::new("project.json");
        load_file(path).expect("Error loading project.json.")
    };
    let project_json =
        serde_json::from_str(&project_json).expect("Error deserializing project.json");

    let schema = serde_json::from_str(SB3_SCHEMA).expect("Error deserializing JSON schema.");
    let schema = JSONSchema::options()
        .with_draft(jsonschema::Draft::Draft4)
        .with_resolver(Sb3Resolver {})
        .compile(&schema)
        .expect("Error compiling JSON schema.");

    let result = schema.validate(&project_json);

    if let Err(errors) = result {
        for error in errors {
            eprintln!("Validation Error: {}", error);
            eprintln!("Instance Path: {}", error.instance_path);
        }
    }
}
