use crate::{
    btree::{
        node::NodeType,
        BTree,
    },
    pager::{
        page::PageIndex,
        Pager,
    },
};

pub struct Engine<'a> {
    pager: Pager<'a>,
}

impl Engine {
    pub fn run(&mut self) {
        let mut btree = BTree::new(&mut self.pager, PageIndex(0));
        for row in btree.open_cursor() {}
    }
}
