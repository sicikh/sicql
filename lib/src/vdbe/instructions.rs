pub struct Op {
    code: Instructions,
}

pub enum Instructions {
    Transaction,
    Halt,
}
