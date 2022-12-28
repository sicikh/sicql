use crate::pager::error::PagerError;

#[derive(Copy, Clone)]
pub struct PageIndex(pub usize);

pub struct Page {
    pub index: PageIndex,
    pub data: Vec<u8>,
}

impl Page {
    pub fn new(data: &[u8], page_size: usize, page_index: PageIndex) -> Result<Self, PagerError> {
        if data.len() != page_size {
            // TODO!
            return Err(PagerError::Generic("TODO".to_string()));
        }
        Ok(Page {
            index: page_index,
            data: data.to_owned(),
        })
    }
}
