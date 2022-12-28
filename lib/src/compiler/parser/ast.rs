use crate::btree::table::row::Value;

pub struct Identifier(pub usize);

pub enum AST {
    WriteTransaction(Vec<Statement>),
    ReadTransaction(Vec<Query>),
}

enum Statement {
    Query {
        projection: Vec<Identifier>,
        from: Vec<TableWithJoins>,
        selection: Option<Expression>,
    },
    Instruction(Instruction),
}

enum Instruction {
    Create {
        existence: Option<bool>,
        column_declarations: Vec<(Identifier, Value)>,
    },
    Drop {
        existence: Option<bool>,
        table: Identifier,
    },
    Insert {
        table: Identifier,
        columns: Vec<Identifier>,
        values: Vec<Value>,
    },
    Update {
        table: Identifier,
        columns_insertions: Vec<(Identifier, Value)>,
        selection: Expression,
    },
    Delete {
        table: Identifier,
        selection: Expression,
    },
}

enum Expression {
    Identifier(Identifier),
    BinaryOperation {
        lhs: Expression,
        op: BinaryOperator,
        rhs: Expression,
    },
    Literal(Value),
}

enum BinaryOperator {
    Plus,
    Minus,
    Divide,
    Multiply,
    Gt,
    Lt,
    GtEq,
    LtEq,
    Eq,
    Neq,
    And,
    Or,
}
