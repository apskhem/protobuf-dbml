use dbml_rs::ast::*;

pub trait ToColType {
  fn to_col_type(&self) -> Option<String>;
}

impl ToColType for table::ColumnType {
  fn to_col_type(&self) -> Option<String> {
    let str_type = match &self.type_name {
      table::ColumnTypeName::Enum(s) => Some(format!("{}", s)),
      table::ColumnTypeName::Char => Some(format!("string")),
      table::ColumnTypeName::VarChar => Some(format!("string")),
      table::ColumnTypeName::SmallInt => Some(format!("int32")),
      table::ColumnTypeName::Integer => Some(format!("int32")),
      table::ColumnTypeName::BigInt => Some(format!("int64")),
      table::ColumnTypeName::Real => Some(format!("float")),
      table::ColumnTypeName::DoublePrecision => Some(format!("double")),
      table::ColumnTypeName::Bool => Some(format!("bool")),
      table::ColumnTypeName::ByteArray => Some(format!("bytes")),
      table::ColumnTypeName::Date => Some(format!("int32")), // string, int32
      table::ColumnTypeName::Text => Some(format!("string")),
      table::ColumnTypeName::Time => Some(format!("int32")), // string, int32
      table::ColumnTypeName::Timestamp => Some(format!("google.protobuf.Timestamp")), // string, int64
      table::ColumnTypeName::Timestamptz => Some(format!("google.protobuf.Timestamp")), // string, int64
      table::ColumnTypeName::Uuid => Some(format!("string")),
      table::ColumnTypeName::Json => Some(format!("string")), // string, bytes, google.protobuf.Struct, google.protobuf.Value google.protobuf.ListValue
      table::ColumnTypeName::Decimal => Some(format!("double")),
      _ => None,
    };

    str_type.and_then(|v| {
      if self.arrays.is_empty() {
        Some(v)
      } else {
        Some(format!("repeated {}", v))
      }
    })
  }
}
