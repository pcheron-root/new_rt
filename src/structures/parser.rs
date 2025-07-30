use std::collections::HashMap;
use std::fs;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
}

imp Parser {
    
    // return name_bloc or None
    fn extract_block_name(&self, line: &str) -> Option<String> {
        if line.ends_with('{') {
            let name = line.trim_end_matches('{').trim();
            if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return Some(name.to_string());
            }
        }
        None
    }

}