use std::{
    collections::BTreeMap,
    fs::File,
    io::{
        Read,
        Seek,
        SeekFrom,
    },
    path::PathBuf,
};

use shared_memory::Shmem;

use self::{
    error::DBConnectionError,
    shm::ShM,
    wal::WAL,
};
use crate::{
    btree::BTree,
    pager::{
        page::{
            Page,
            PageIndex,
        },
        Pager,
    },
};

mod error;
pub mod shm;
pub mod wal;

pub struct DBMetadata {
    pub page_size: PageSize,
}

pub struct DBFile(pub File);

pub struct DBInMemory {
    pub metadata: DBMetadata,
    pub data: Vec<u8>,
    pub cursor: usize,
}

pub enum DBConnection {
    Memory(DBInMemory),
    Disk {
        metadata: DBMetadata,
        db_file: DBFile,
        wal: WAL,
        shm: ShM,
    },
}

pub enum Location {
    Memory,
    Disk(PathBuf),
}

pub enum PageSize {
    B4096,
    B8192,
    B16384,
    B32768,
    B65536,
}

impl PageSize {
    pub fn in_power_of_two(&self) -> u8 {
        match self {
            PageSize::B4096 => 12,
            PageSize::B8192 => 13,
            PageSize::B16384 => 14,
            PageSize::B32768 => 15,
            PageSize::B65536 => 16,
        }
    }

    pub fn value(&self) -> usize {
        match self {
            PageSize::B4096 => 4096,
            PageSize::B8192 => 8192,
            PageSize::B16384 => 16384,
            PageSize::B32768 => 32786,
            PageSize::B65536 => 65536,
        }
    }
}

impl DBConnection {
    pub fn new(location: Location, page_size: PageSize) -> Result<Self, DBConnectionError> {
        match location {
            Location::Memory => {
                /* let db = DBInMemory {
                        metadata: DBMetadata {},
                        data: Vec::with_capacity(),
                        cursor: 0,

                Ok(DBConnection::Memory()) */
                todo!()
            },
            Location::Disk(path_buf) => {
                todo!()
            },
        }
    }

    pub fn metadata(&self) -> &DBMetadata {
        match self {
            DBConnection::Memory(db) => &db.metadata,
            DBConnection::Disk { metadata, .. } => metadata,
        }
    }

    pub fn execute(&mut self) -> Result<(), DBConnectionError> {
        let pager = Pager::new(self)?;
        let btree = BTree::new(pager)?;

        todo!()
    }
}
impl Drop for DBConnection {
    fn drop(&mut self) {}
}
