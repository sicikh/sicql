use self::{
    error::BTreeError,
    node::Node,
};
use crate::{
    btree::{
        node::NodeType,
        table::{
            row::Row,
            TableNode,
        },
    },
    pager::{
        page::{
            Page,
            PageIndex,
        },
        Pager,
    },
};

mod error;
// TODO: Not implemented
// mod index;
pub mod node;
pub mod table;

pub struct BTree<'a> {
    pager: &'a mut Pager<'a>,
    root: PageIndex,
}

impl<'a> BTree<'a> {
    pub fn new(pager: &mut Pager, root_index: PageIndex) -> Self {
        BTree {
            pager,
            root: root_index,
        }
    }

    pub fn open_cursor(&mut self) -> Cursor {
        Cursor(self)
    }
}

pub struct Cursor<'a>(&'a BTree<'a>);

impl Iterator for Cursor {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
