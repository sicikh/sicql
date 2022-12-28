pub mod error;
pub mod page;

use std::{
    collections::BTreeMap,
    fs::File,
    io::{
        Read,
        Seek,
        SeekFrom,
    },
};

use self::{
    error::PagerError,
    page::{
        Page,
        PageIndex,
    },
};
use crate::db_connection::{
    wal::FrameIndex,
    DBConnection,
};

pub struct Pager<'a> {
    db_connection: &'a mut DBConnection,
    mark: FrameIndex,
}

impl<'a> Pager<'a> {
    pub fn new(db_connection: &'a mut DBConnection) -> Result<Self, PagerError> {
        Ok(Pager {
            mark: match db_connection {
                DBConnection::Memory(_) => FrameIndex(0),
                DBConnection::Disk { wal, .. } => wal.get_last_frame_index(),
            },
            db_connection,
        })
    }

    pub fn get_page(&self, index: PageIndex) -> Result<Page, PagerError> {
        match self.db_connection {
            DBConnection::Memory(db) => {
                // Max size of in-memory DB is 4 GB, don't allow indexing out of bounds
                if index.0 > (u32::MAX >> db.metadata.page_size.in_power_of_two()) as usize {
                    // TODO!
                    return Err(PagerError::Generic("TODO".to_string()));
                }
                let page_size = db.metadata.page_size.value();

                Ok(Page::new(
                    &db.data[index.0 * page_size..index.0 * page_size + page_size],
                    page_size,
                    index,
                )?)
            },
            DBConnection::Disk {
                db_file,
                wal,
                shm,
                metadata,
            } => {
                let mut page = match shm.get_page_index_by_frame(&self.mark) {
                    // WAL-hit
                    Some(page_index) => wal.get_page(page_index)?,
                    // WAL-miss
                    None => {
                        let page_size = metadata.page_size.value();

                        let mut raw_page: Vec<u8> = Vec::with_capacity(page_size);
                        db_file
                            .0
                            .seek(SeekFrom::Start((index.0 * page_size) as u64))?;
                        db_file.0.read_exact(&mut raw_page);
                        Page::new(&raw_page, page_size, index)?
                    },
                };

                Ok(page)
            },
        }
    }

    pub fn write_page(&self, page: Page) -> Result<(), PagerError> {
        todo!()
    }

    pub fn request_free_page_index(&self) -> Result<PageIndex, PagerError> {
        todo!()
    }

    pub fn wal_checkpoint(&self) {
        todo!()
    }

    pub fn begin_write_transaction(&mut self) {
        todo!()
    }

    pub fn abort_write_transaction(&mut self) {
        todo!()
    }

    pub fn commit_write_transaction(&mut self) {
        todo!()
    }

    pub fn begin_read_transaction(&self) {
        todo!()
    }

    pub fn abort_read_transaction(&self) {
        todo!()
    }

    pub fn commit_read_transaction(&self) {
        todo!()
    }
}
