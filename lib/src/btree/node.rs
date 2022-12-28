use super::table::TableNode;
use crate::{
    btree::{
        error::BTreeError,
        table::row::RowID,
    },
    pager::{
        page::{
            Page,
            PageIndex,
        },
        Pager,
    },
};

pub struct Node {
    pub index: PageIndex,
    // TODO!
    // pub empty_space: u64
    pub type_of: NodeType,
}

pub enum NodeType {
    TableNode(TableNode), // TODO: IndexNode
}

impl Node {
    pub fn new(page_index: PageIndex, node_type: NodeType) -> Self {
        Node {
            index: page_index,
            type_of: node_type,
        }
    }

    pub fn is_full(&self, branching_factor: usize) -> bool {
        match &self.type_of {
            NodeType::TableNode(node) => {
                match node {
                    TableNode::Interior { rid_keys, .. } => {
                        todo!()
                    },
                    TableNode::Leaf { .. } => {
                        todo!()
                    },
                }
            },
        }
    }

    pub fn split(
        &mut self,
        branching_factor: usize,
        pager: &mut Pager,
    ) -> Result<(RowID, Node), BTreeError> {
        let mut free_page_index = pager.request_free_page_index()?;

        match &self.type_of {
            NodeType::TableNode(node) => {
                match node {
                    TableNode::Interior {
                        mut is_root,
                        mut rid_keys,
                        mut children,
                    } => {
                        let mut siblings_keys = rid_keys.split_off(branching_factor);
                        let key_to_delegate = siblings_keys
                            .last()
                            .ok_or(BTreeError::Generic("Unexpected".to_string()))?
                            .clone();
                        rid_keys.pop();

                        let mut siblings_children = children.split_off(branching_factor);
                        let siblings_children = children.split_off(branching_factor);

                        is_root = false;

                        Ok((
                            key_to_delegate,
                            Node::new(
                                free_page_index,
                                NodeType::TableNode(TableNode::Interior {
                                    is_root: false,
                                    rid_keys: siblings_keys,
                                    children: siblings_children,
                                }),
                            ),
                        ))
                    },
                    TableNode::Leaf {
                        mut rows,
                        mut next_node_ptr,
                    } => {
                        let siblings = rows.split_off(branching_factor);

                        // FIXME: error handling
                        let last_row = rows
                            .last()
                            .ok_or(BTreeError::Generic("Unexpected".to_string()))?;

                        let key_to_delegate = last_row.clone().id;

                        let prev_next_node_ptr = next_node_ptr.clone();
                        next_node_ptr = Some(free_page_index);

                        Ok((
                            key_to_delegate,
                            Node::new(
                                free_page_index,
                                NodeType::TableNode(TableNode::Leaf {
                                    rows: siblings,
                                    next_node_ptr: prev_next_node_ptr,
                                }),
                            ),
                        ))
                    },
                }
            },
        }
    }
}

impl TryFrom<Page> for Node {
    type Error = ();

    fn try_from(value: Page) -> Result<Self, Self::Error> {
        todo!()
    }
}
