use std::{ffi::OsString, error::Error};

use inflector::Inflector;

use super::transpile;

/// Protobuf target version.
#[derive(Debug, PartialEq, Clone)]
pub enum Target {
  Proto3
}

/// Configuration options for the code generation.
#[derive(Debug, PartialEq, Clone)]
pub struct Config  {
  /// Input file path.
  pub in_path: OsString,
  /// Output file path.
  pub out_path: OsString,
  /// Protobuf target version.
  pub target: Target,
  /// Output package name
  pub package_name: String,
  /// A function for transforming the original table name from DBML.
  /// The default funciton is pascal casing and appending "Schema" to the name.
  pub table_name_transform_fn: fn(Option<&str>, &str, Option<&str>) -> String,
  /// A function for transforming the original enum name from DBML.
  /// The default funciton is pascal casing and appending "Schema" to the name.
  pub enum_name_transform_fn: fn(Option<&str>, &str) -> String,
  /// Output update schemas with optional all fields by default, entity's primary key field is omitted.
  pub is_with_update_schema: bool,
  /// Ouput create schemas with optional default fields.
  /// The entity's primary key field is included when it is not being auto generated.
  pub is_with_create_schema: bool,
  /// Forcing `is_with_create_schema` wether it should include primary key field or not.
  pub is_create_schema_primary_key_included: Option<bool>,
  /// Config `is_with_update_schema` to include primary key field.
  pub is_update_schema_primary_key_included: bool
}

impl Default for Config {
  fn default() -> Self {
    Self {
      in_path: OsString::from(""),
      out_path: OsString::from(""),
      target: Target::Proto3,
      package_name: String::new(),
      table_name_transform_fn: |_: Option<&str>, name: &str, _: Option<&str>| format!("{}Schema", name.to_pascal_case()),
      enum_name_transform_fn: |_: Option<&str>, name: &str| format!("{}Enum", name.to_pascal_case()),
      is_with_update_schema: false,
      is_with_create_schema: false,
      is_create_schema_primary_key_included: None,
      is_update_schema_primary_key_included: false
    }
  }
}

impl Config {
  pub fn validate(&self) -> Option<&str> {
    if self.in_path.is_empty() {
      Some("in_path is not set")
    } else if self.out_path.is_empty() {
      Some("out_path is not set")
    } else {
      None
    }
  }

  pub fn transpile(&self) -> Result<String, Box<dyn Error>> {
    let sem_ast = dbml_rs::parse_file(&self.in_path)?;
    
    let result = transpile(sem_ast, &self)?;

    Ok(result)
  }
}