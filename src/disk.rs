use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, SeekFrom};
use std::path::Path;

pub const PAGE_SIZE: usize = 4096;

pub struct PageId(pub u64);

impl PageId {
    pub const INVALID_PAGE_ID: PageId = PageId(u64::MAX);

    pub fn to_u64(self) -> u64 { self.0 }
}

pub struct DiskManager {
    heap_file: File,
    next_page_id: u64,
}

impl DiskManager {
    pub fn new(heap_file: File) -> io::Result<Self> {
        let heap_file_size = heap_file.metadata()?.len();
        let next_page_id = heap_file_size / PAGE_SIZE as u64;

        Ok(Self {
            heap_file,
            next_page_id,
        })
    }

    pub fn open(data_file_path: impl AsRef<Path>) -> io::Result<Self> {
        let heap_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(data_file_path)?;
        Self::new(heap_file)
    }

    pub fn allocate_page(&mut self) -> PageId {
        let page_id = self.next_page_id;
        self.next_page_id += 1;
        PageId(page_id)
    }

    pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {
        self.seek(page_id)?;
        self.heap_file.read_exact(data)
    }

    pub fn write_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {
        self.seek(page_id)?;
        self.heap_file.write_all(data)
    }

    fn offset(&self, page_id: PageId) -> u64 {
        PAGE_SIZE as u64 * page_id.to_u64()
    }
    fn seek(&mut self, page_id: PageId) -> io::Result<()> {
        self.heap_file.seek(SeekFrom::Start(self.offset(page_id)))?;
        Ok(())
    }
}
