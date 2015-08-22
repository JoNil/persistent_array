extern crate memmap;

use std::default::Default;
use std::fs::File;
use std::io::{self, Write};
use std::marker::PhantomData;
use std::mem::{transmute, size_of};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::slice;

use memmap::{Mmap, Protection};

#[derive(Debug)]
pub enum Error {
    UnableToOpenFile(io::Error),
    //WrongMagicBytes,
    //WrongTypeId,
    //WrongFileSize,
}

/// Persistent Array
///
/// A memory mapped array that can be used as a slice.
pub struct PersistentArray<T> where T: Copy + Default {
    memory_map: Mmap,
    memory_type: PhantomData<T>,
}

impl<T> PersistentArray<T> where T: Copy + Default {

    /// Creates a new persistent array
    pub fn new<P>(path: P, size: u64) -> Result<PersistentArray<T>, Error>
            where P: AsRef<Path> {
        
        {
            let mut file = match File::create(&path) {
                Ok(file) => file,
                Err(err) => return Err(Error::UnableToOpenFile(err)),
            };

            let element: T = Default::default();
            let element_arr: &[u8] = unsafe { slice::from_raw_parts(transmute(&element), size_of::<T>()) }; 

            for _ in 0..size {
                match file.write(element_arr) {
                    Ok(written) => {
                        if written != element_arr.len() {
                            return Err(Error::UnableToOpenFile(io::Error::new(
                                       io::ErrorKind::Other, "Unable to write initial data")));
                        }
                    },
                    Err(err) => return Err(Error::UnableToOpenFile(err)),
                };
            }
        }

        let mmap = match Mmap::open_path(&path, Protection::ReadWrite) {
            Ok(mmap) => mmap,
            Err(err) => return Err(Error::UnableToOpenFile(err)),
        };

        Ok(PersistentArray {
            memory_map: mmap,
            memory_type: PhantomData,
        })
    }

    /// Opens an existing persistent array
    pub fn open<P>(path: P) -> Result<PersistentArray<T>, Error>
            where P: AsRef<Path> {

        let mmap = match Mmap::open_path(&path, Protection::ReadWrite) {
            Ok(mmap) => mmap,
            Err(err) => return Err(Error::UnableToOpenFile(err)),
        };

        Ok(PersistentArray {
            memory_map: mmap,
            memory_type: PhantomData,
        })
    }
}

impl<T> Deref for PersistentArray<T> where T: Copy + Default {
    type Target = [T];

    fn deref(&self) -> &[T] {

        let ptr = self.memory_map.ptr();
        let length = self.memory_map.len() / size_of::<T>();

        unsafe { slice::from_raw_parts(ptr as *const T, length) }
    }
}

impl<T> DerefMut for PersistentArray<T> where T: Copy + Default {

    fn deref_mut(&mut self) -> &mut [T] {

        let ptr = self.memory_map.mut_ptr();
        let length = self.memory_map.len() / size_of::<T>();

        unsafe { slice::from_raw_parts_mut(ptr as *mut T, length) }
    }
}