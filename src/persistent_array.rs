// http://tomaka.github.io/glium//mmap/index.html

use memmap::{Mmap, Protection};
use std::default::Default;
use std::path::Path;

#[derive(Debug)]
pub enum PersistentArrayError {
    UnableToOpenFile,
    WrongMagicBytes,
    WrongTypeId,
    WrongFileSize,
}

pub struct PersistentArray<T> where T: Copy + Default {
    a: u64,
    b: T,
}

impl<T> PersistentArray<T> where T: Copy + Default {

    pub fn new<P>(path: P, size: u64) -> Result<PersistentArray<T>, PersistentArrayError>
            where P: AsRef<Path> {
        

        //let mmap = Mmap::open_path(path, Protection::Read).unwrap();



        Ok(PersistentArray {
            a: 10,
            b: Default::default(),
        })
    }

    pub fn open<P>(path: P) -> Result<PersistentArray<T>, PersistentArrayError>
            where P: AsRef<Path> {

        Ok(PersistentArray {
            a: 10,
            b: Default::default(),
        })
    }
}