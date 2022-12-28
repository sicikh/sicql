use shared_memory::Shmem;

use crate::{
    db_connection::wal::{
        FrameIndex,
        WAL,
    },
    pager::page::PageIndex,
};

pub struct ShM {
    file: Shmem,
}

impl ShM {
    pub fn new() -> Self {
        todo!()
    }

    pub fn get_page_index_by_frame(&self, frame_index: &FrameIndex) -> Option<PageIndex> {
        todo!()
    }

    pub fn update_index(&self, wal: &WAL) -> Result<(), std::io::Error> {
        todo!()
    }
}

impl Default for ShM {
    fn default() -> Self {
        todo!()
    }
}
