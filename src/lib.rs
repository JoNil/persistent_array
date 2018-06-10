// Copyright (c) 2016 Jonathan Nilsson
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

extern crate layout_id;
extern crate memmap;

use layout_id::layout_id;
use memmap::MmapMut;
use memmap::MmapOptions;
use std::default::Default;
use std::fs::OpenOptions;
use std::io;
use std::marker::PhantomData;
use std::mem::{size_of, transmute};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::slice;

const MAGIC_BYTES: &'static [u8; 4] = b"PADB";

#[derive(Debug)]
pub enum Error {
    UnableToOpenFile(io::Error),
    WrongMagicBytes,
    WrongFileSize,
    WrongTypeId,
}

/// Persistent Array
///
/// A memory mapped array that can be used as a slice.
pub struct PersistentArray<T: Copy> {
    phantom_type: PhantomData<T>,
    map: MmapMut,
    elements: u64,
}

#[repr(C, packed)]
struct Header {
    magic: [u8; 4],
    size: u64,
    typeid: u64,
}

impl<T: Copy + Default> PersistentArray<T> {
    /// Creates a new persistent array
    pub fn new<P>(path: P, size: u64) -> Result<PersistentArray<T>, Error>
    where
        P: AsRef<Path>,
    {
        let file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)
        {
            Ok(file) => file,
            Err(err) => return Err(Error::UnableToOpenFile(err)),
        };

        if let Err(err) = file.set_len(size * size_of::<T>() as u64 + size_of::<Header>() as u64) {
            return Err(Error::UnableToOpenFile(err));
        }

        println!("{:?}", "Hej");

        let mut map = unsafe {
            match MmapOptions::new().map_mut(&file) {
                Ok(map) => map,
                Err(err) => return Err(Error::UnableToOpenFile(err)),
            }
        };

        println!("{:?}", "Hej");

        if map.len() as u64 != size * size_of::<T>() as u64 + size_of::<Header>() as u64 {
            return Err(Error::WrongFileSize);
        }

        let header: &mut Header = unsafe { transmute(map.as_mut_ptr()) };

        *header = Header {
            magic: *MAGIC_BYTES,
            size: size,
            typeid: layout_id::<T>(),
        };

        let element: T = Default::default();

        let elements: &mut [T] = unsafe {
            slice::from_raw_parts_mut(
                map.as_mut_ptr().offset(size_of::<Header>() as isize) as *mut T,
                size as usize,
            )
        };

        for e in elements.iter_mut() {
            *e = element;
        }

        Ok(PersistentArray {
            phantom_type: PhantomData,
            map: map,
            elements: size,
        })
    }

    /// Opens an existing persistent array
    pub fn open<P>(path: P) -> Result<PersistentArray<T>, Error>
    where
        P: AsRef<Path>,
    {
        let file = match OpenOptions::new().read(true).write(true).open(&path) {
            Ok(file) => file,
            Err(err) => return Err(Error::UnableToOpenFile(err)),
        };

        let map = unsafe {
            match MmapOptions::new().map_mut(&file) {
                Ok(map) => map,
                Err(err) => return Err(Error::UnableToOpenFile(err)),
            }
        };

        let ptr = map.as_ptr();
        let size = map.len();

        if size < size_of::<Header>() {
            return Err(Error::WrongFileSize);
        }

        let header: &Header = unsafe { transmute(ptr) };

        if header.magic != *MAGIC_BYTES {
            return Err(Error::WrongMagicBytes);
        }

        let elements = ((size - size_of::<Header>()) / size_of::<T>()) as u64;

        if header.size != elements {
            return Err(Error::WrongFileSize);
        }

        if header.typeid != layout_id::<T>() {
            return Err(Error::WrongTypeId);
        }

        Ok(PersistentArray {
            phantom_type: PhantomData,
            map: map,
            elements: elements,
        })
    }
}

impl<T: Copy> Deref for PersistentArray<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(
                self.map.as_ptr().offset(size_of::<Header>() as isize) as *const T,
                self.elements as usize,
            )
        }
    }
}

impl<T: Copy> DerefMut for PersistentArray<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(
                self.map.as_mut_ptr().offset(size_of::<Header>() as isize) as *mut T,
                self.elements as usize,
            )
        }
    }
}
