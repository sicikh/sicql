use crate::{
    btree::table::row::{
        Row,
        RowID,
    },
    pager::page::PageIndex,
};

pub mod row;

pub enum TableNode {
    Interior {
        is_root: bool,
        rid_keys: Vec<RowID>,
        children: Vec<PageIndex>,
    },
    Leaf {
        rows: Vec<Row>,
        next_node_ptr: Option<PageIndex>,
    },
}
