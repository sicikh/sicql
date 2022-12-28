use std::fs::File;

use crate::pager::page::{
    Page,
    PageIndex,
};

pub struct FrameIndex(pub usize);

pub struct WAL {
    pub file: File,
}

impl WAL {
    pub fn get_last_frame_index(&self) -> FrameIndex {
        todo!()
    }

    pub fn begin_frame(&mut self) -> Result<(), std::io::Error> {
        todo!()
    }

    pub fn close_frame(&mut self) -> Result<(), std::io::Error> {
        todo!()
    }

    pub fn get_page(&self, page_index: PageIndex) -> Result<Page, std::io::Error> {
        todo!()
    }
}

pub struct Frame {}
