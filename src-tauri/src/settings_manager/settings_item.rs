use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Type, Default)]
pub struct Table {
    pub data: HashMap<String, Value>,
}

impl Table {
    pub fn from_toml_table(raw: toml::Table) -> Result<Self, String> {
        let mut data: HashMap<String, Value> = HashMap::new();
        for (key, value) in raw {
            data.insert(key, Value::from_toml_value(value)?);
        }
        Ok(Self { data })
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Type)]
pub enum Value {
    /// Represents a TOML string
    String(String),
    /// Represents a TOML integer
    Integer(i64),
    /// Represents a TOML float
    Float(f64),
    /// Represents a TOML boolean
    Boolean(bool),
    /// Represents a TOML array
    Array(Vec<Value>),
    /// Represents a TOML table
    Table(Table),
}

impl Value {
    pub fn from_toml_value(raw: toml::Value) -> Result<Self, String> {
        let var_name: Self = match raw {
            toml::Value::Datetime(_) => {
                return Err("Cannot create value object from this toml::value".to_string());
            }
            toml::Value::String(str_v) => Self::String(str_v),
            toml::Value::Boolean(bool_v) => Self::Boolean(bool_v),
            toml::Value::Array(arr) => {
                let mut result: Vec<Self> = Vec::new();
                for element in arr {
                    result.push(Self::from_toml_value(element).unwrap());
                }
                Self::Array(result)
            }
            toml::Value::Integer(integer_v) => Self::Integer(integer_v),
            toml::Value::Float(float_v) => Self::Float(float_v),
            toml::Value::Table(table_v) => {
                let mut result: HashMap<String, Self> = HashMap::new();
                for (str_v, value_v) in table_v {
                    result.insert(str_v.clone(), Self::from_toml_value(value_v).unwrap());
                }
                Self::Table(Table { data: result })
            }
        };
        Ok(var_name)
    }
}

#[derive(Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SettingItems {
    pub items: Table,
}
