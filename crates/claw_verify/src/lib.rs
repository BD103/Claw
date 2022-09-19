#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

use anyhow::anyhow;
use jsonschema::{
    error::ValidationErrorKind, paths::JSONPointer, JSONSchema, SchemaResolver, SchemaResolverError,
};
use serde_json::Value;
use std::sync::Arc;
use url::Url;

/// The schema that a `project.json` file is checked against.
pub const SB3_SCHEMA: &str = include_str!("sb3_schema.json");

/// Shared definitions between multiple schemas, though right now just [`SB3_SCHEMA`].
pub const SB3_DEFINITIONS: &str = include_str!("sb3_definitions.json");

/// Hacky workaround so that the schema actually works. Most likely this will be re-written in the
/// future.
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

/// Returns the compiled [`JSONSchema`] of the project JSON.
///
/// # Panics
///
/// Technically this function can panic if the schema is malformed, but the schema is unlikely to
/// change and will be tested with before releasing.
fn compile_schema() -> JSONSchema {
    // Since the schema never changes, it is safe to unwrap here.
    let schema: Value = serde_json::from_str(SB3_SCHEMA).expect("Error deserializing JSON schema.");

    JSONSchema::options()
        .with_draft(jsonschema::Draft::Draft4)
        .with_resolver(Sb3Resolver {})
        .compile(&schema)
        .expect("Error compiling JSON schema.")
}

/// The main function that verifies a [`serde_json::Value`] is a valid project JSON.
pub fn verify(project_json: &Value) -> Result<(), Vec<(ValidationErrorKind, JSONPointer)>> {
    let schema = compile_schema();
    let res = schema.validate(project_json);

    res.map_err(|e| e.into_iter().map(|x| (x.kind, x.instance_path)).collect())
}

/// Conveniance function that verifies the string form of a project JSON.
///
/// # Panics
///
/// This function **will** panic if you give it an invalid JSON string. If you are worried about
/// catching these errors, please use [`verify`] directly.
pub fn verify_string(project_json: &str) -> Result<(), Vec<(ValidationErrorKind, JSONPointer)>> {
    verify(&serde_json::from_str(project_json).expect("Error deserializing project JSON."))
}
