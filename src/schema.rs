use crate::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::{clone::Clone, str::FromStr};

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SchemaData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_only: Option<bool>,
    /// Specifies that a schema is deprecated and SHOULD be transitioned out
    /// of usage. Default value is `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    /// Additional external documentation for this schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Adds support for polymorphism. The discriminator is the schema property
    /// name that is used to differentiate between other schema that inherit
    /// this schema. The property name used MUST be defined at this schema and
    /// it MUST be in the `required` property list. When used, the value MUST be
    ///  the name of this schema or any schema that inherits it. See
    /// [Composition and Inheritance](https://www.asyncapi.com/docs/specifications/v2.6.0#schemaComposition)
    /// for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Schema {
    #[serde(flatten)]
    pub schema_data: SchemaData,
    #[serde(flatten)]
    pub schema_kind: SchemaKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SchemaKind {
    Type(Type),
    OneOf {
        #[serde(rename = "oneOf")]
        one_of: Vec<ReferenceOr<Schema>>,
    },
    AllOf {
        #[serde(rename = "allOf")]
        all_of: Vec<ReferenceOr<Schema>>,
    },
    AnyOf {
        #[serde(rename = "anyOf")]
        any_of: Vec<ReferenceOr<Schema>>,
    },
    Any(AnySchema),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Type {
    String(StringType),
    Number(NumberType),
    Integer(IntegerType),
    Object(ObjectType),
    Array(ArrayType),
    Boolean {},
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AdditionalProperties {
    Any(bool),
    Schema(Box<ReferenceOr<Schema>>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AnySchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub properties: IndexMap<String, ReferenceOr<Box<Schema>>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<AdditionalProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ReferenceOr<Box<Schema>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StringType {
    #[serde(default, skip_serializing_if = "VariantOrUnknownOrEmpty::is_empty")]
    pub format: VariantOrUnknownOrEmpty<StringFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")]
    pub enumeration: Vec<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NumberType {
    #[serde(default, skip_serializing_if = "VariantOrUnknownOrEmpty::is_empty")]
    pub format: VariantOrUnknownOrEmpty<NumberFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")]
    pub enumeration: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IntegerType {
    #[serde(default, skip_serializing_if = "VariantOrUnknownOrEmpty::is_empty")]
    pub format: VariantOrUnknownOrEmpty<IntegerFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    #[serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")]
    pub enumeration: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ObjectType {
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub properties: IndexMap<String, ReferenceOr<Box<Schema>>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<AdditionalProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<usize>,
}

impl ObjectType {
    pub fn GetPropertie(typ: String, ex_value: String, description: String) -> ReferenceOr<Box<Schema>> {
        let res = match typ.as_str() {
            "u32" => ObjectType::CreatePropertie(ex_value.parse::<i32>().unwrap_or(0), description),
            "u64" => ObjectType::CreatePropertie(ex_value.parse::<i64>().unwrap_or(0), description),
            "i32" => ObjectType::CreatePropertie(ex_value.parse::<i32>().unwrap_or(0), description),
            "i64" => ObjectType::CreatePropertie(ex_value.parse::<i64>().unwrap_or(0), description),
            "f32" => ObjectType::CreatePropertie(ex_value.parse::<f32>().unwrap_or(0.0), description),
            "f64" => ObjectType::CreatePropertie(ex_value.parse::<f64>().unwrap_or(0.0), description),
            "String" => ObjectType::CreatePropertie(ex_value, description),
            _ => {
                let mut schema = Schema {
                    schema_kind: SchemaKind::Any(AnySchema {
                        format: Some(typ),
                        ..Default::default()
                    }),
                    schema_data: SchemaData {
                        nullable: None,
                        read_only: None,
                        write_only: None,
                        deprecated: None,
                        external_docs: None,
                        example: Some(serde_json::Value::from_str(ex_value.as_str()).unwrap()),
                        title: None,
                        description: Some(description.clone()),
                        discriminator: None,
                        default: None,
                    },
                };
                ReferenceOr::Item(Box::new(schema))
            }
        };
        return res;
    }
}

pub trait VValue<T> {
    fn CreatePropertie(v_int: T, description: String) -> ReferenceOr<Box<Schema>>;
}
impl VValue<i32> for ObjectType {
    fn CreatePropertie(v_int: i32, description: String) -> ReferenceOr<Box<Schema>> {
        let mut schema = Schema {
            schema_kind: SchemaKind::Type(Type::Integer(IntegerType {
                format: VariantOrUnknownOrEmpty::Item(IntegerFormat::Int32),
                multiple_of: None,
                exclusive_minimum: None,
                exclusive_maximum: None,
                minimum: None,
                maximum: None,
                enumeration: Vec::new(),
            })),
            schema_data: SchemaData {
                nullable: None,
                read_only: None,
                write_only: None,
                deprecated: None,
                external_docs: None,
                example: Some(serde_json::Value::from(v_int)),
                title: None,
                description: Some(description.clone()),
                discriminator: None,
                default: None,
            },
        };
        ReferenceOr::Item(Box::new(schema))
    }
}
impl VValue<i64> for ObjectType {
    fn CreatePropertie(v_int: i64, description: String) -> ReferenceOr<Box<Schema>> {
        let mut schema = Schema {
            schema_kind: SchemaKind::Type(Type::Integer(IntegerType {
                format: VariantOrUnknownOrEmpty::Item(IntegerFormat::Int64),
                multiple_of: None,
                exclusive_minimum: None,
                exclusive_maximum: None,
                minimum: None,
                maximum: None,
                enumeration: Vec::new(),
            })),
            schema_data: SchemaData {
                nullable: None,
                read_only: None,
                write_only: None,
                deprecated: None,
                external_docs: None,
                example: Some(serde_json::Value::from(v_int)),
                title: None,
                description: Some(description.clone()),
                discriminator: None,
                default: None,
            },
        };
        ReferenceOr::Item(Box::new(schema))
    }
}

impl VValue<f32> for ObjectType {
    fn CreatePropertie(v_number: f32, description: String) -> ReferenceOr<Box<Schema>> {
        let mut schema = Schema {
            schema_kind: SchemaKind::Type(Type::Number(NumberType {
                format: VariantOrUnknownOrEmpty::Item(NumberFormat::Float),
                multiple_of: None,
                exclusive_minimum: None,
                exclusive_maximum: None,
                minimum: None,
                maximum: None,
                enumeration: Vec::new(),
            })),
            schema_data: SchemaData {
                nullable: None,
                read_only: None,
                write_only: None,
                deprecated: None,
                external_docs: None,
                example: Some(serde_json::Value::from(v_number)),
                title: None,
                description: Some(description.clone()),
                discriminator: None,
                default: None,
            },
        };
        ReferenceOr::Item(Box::new(schema))
    }
}

impl VValue<f64> for ObjectType {
    fn CreatePropertie(v_number: f64, description: String) -> ReferenceOr<Box<Schema>> {
        let mut schema = Schema {
            schema_kind: SchemaKind::Type(Type::Number(NumberType {
                format: VariantOrUnknownOrEmpty::Item(NumberFormat::Double),
                multiple_of: None,
                exclusive_minimum: None,
                exclusive_maximum: None,
                minimum: None,
                maximum: None,
                enumeration: Vec::new(),
            })),
            schema_data: SchemaData {
                nullable: None,
                read_only: None,
                write_only: None,
                deprecated: None,
                external_docs: None,
                example: Some(serde_json::Value::from(v_number)),
                title: None,
                description: Some(description.clone()),
                discriminator: None,
                default: None,
            },
        };
        ReferenceOr::Item(Box::new(schema))
    }
}

impl VValue<String> for ObjectType {
    fn CreatePropertie(v_str: String, description: String) -> ReferenceOr<Box<Schema>> {
        let mut schema = Schema {
            schema_kind: SchemaKind::Type(Type::String(StringType {
                format: VariantOrUnknownOrEmpty::Item(StringFormat::Byte),
                ..Default::default()
            })),
            schema_data: SchemaData {
                nullable: None,
                read_only: None,
                write_only: None,
                deprecated: None,
                external_docs: None,
                example: Some(serde_json::Value::from(v_str)),
                title: None,
                description: Some(description.clone()),
                discriminator: None,
                default: None,
            },
        };
        ReferenceOr::Item(Box::new(schema))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ArrayType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ReferenceOr<Box<Schema>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,
    #[serde(default, skip_serializing_if = "Clone::clone")]
    pub unique_items: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NumberFormat {
    Float,
    Double,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IntegerFormat {
    Int32,
    Int64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StringFormat {
    Date,
    #[serde(rename = "date-time")]
    DateTime,
    Password,
    Byte,
    Binary,
}
