use std::fmt::{
    Display,
    Formatter,
};

#[derive(Copy, Clone)]
pub struct Row {
    pub deleted: bool,
    pub id: RowID,
    pub values: Vec<Value>,
}

#[derive(Copy, Clone)]
pub struct RowID(pub u64);

// TODO: docs
pub enum Value {
    Varchar(String, u64),
    Bool(bool),
    Integer(i64),
    Float(f64),
    Null,
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Value::*;
        todo!()
    }
}
