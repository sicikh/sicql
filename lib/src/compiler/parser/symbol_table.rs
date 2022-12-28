pub struct SymbolTable(pub Vec<ObjectName>);

pub enum ObjectName {
    Column {
        table: ObjectName::Table,
        name: String,
    },
    Table(String),
}
