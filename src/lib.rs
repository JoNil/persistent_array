#![feature(reflect_marker)]

extern crate memmap;

use std::any::TypeId;
use std::default::Default;
use std::fs::File;
use std::hash::{Hash, Hasher, SipHasher};
use std::io;
use std::marker::{PhantomData, Reflect};
use std::mem::{transmute, size_of};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::slice;

use memmap::{Mmap, Protection};

const MAGIC_BYTES: &'static [u8; 4] = b"PADB";

#[derive(Debug)]
pub enum Error {
    UnableToOpenFile(io::Error),
    WrongMagicBytes,
    WrongFileSize,
    WrongTypeId,
}

pub trait TypeBounds: Copy + Default + Reflect + 'static {}
impl<T: Copy + Default + Reflect + 'static> TypeBounds for T {}

/// Persistent Array
///
/// A memory mapped array that can be used as a slice.
pub struct PersistentArray<T: TypeBounds> {
    phantom_type: PhantomData<T>,
    map: Mmap,
    elements: u64,
}

#[repr(C, packed)]
struct Header {
    magic: [u8; 4],
    size: u64,
    typeid: u64,
}

fn get_hashed_type_id<T: Reflect + 'static>() -> u64 {
    let id = TypeId::of::<T>();
    let mut s = SipHasher::new();
    id.hash(&mut s);
    s.finish()
}

impl<T: TypeBounds> PersistentArray<T> {

    /// Creates a new persistent array
    pub fn new<P>(path: P, size: u64) -> Result<PersistentArray<T>, Error>
            where P: AsRef<Path> {
        {
            let file = match File::create(&path) {
                Ok(file) => file,
                Err(err) => return Err(Error::UnableToOpenFile(err)),
            };

            if let Err(err) = file.set_len(size * size_of::<T>() as u64 + size_of::<Header>() as u64) {
                return Err(Error::UnableToOpenFile(err));
            }
        }

        let mut map = match Mmap::open_path(&path, Protection::ReadWrite) {
            Ok(map) => map,
            Err(err) => return Err(Error::UnableToOpenFile(err)),
        };

        if map.len() as u64 != size * size_of::<T>() as u64 + size_of::<Header>() as u64 {
            return Err(Error::WrongFileSize);
        }

        let header: &mut Header = unsafe { transmute(map.mut_ptr()) };

        *header = Header {
            magic: *MAGIC_BYTES,
            size: size,
            typeid: get_hashed_type_id::<T>(),
        };

        let element: T = Default::default();

        let elements: &mut [T] = unsafe {
            slice::from_raw_parts_mut(map.mut_ptr().offset(size_of::<Header>() as isize) as *mut T,
                                      size as usize)
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
            where P: AsRef<Path> {

        let map = match Mmap::open_path(&path, Protection::ReadWrite) {
            Ok(map) => map,
            Err(err) => return Err(Error::UnableToOpenFile(err)),
        };

        let ptr = map.ptr();
        let size = map.len();

        if size < size_of::<Header>() {
            return Err(Error::WrongFileSize);
        }

        let header: &Header = unsafe { transmute(ptr) };

        if header.magic != *MAGIC_BYTES {
            return Err(Error::WrongMagicBytes)
        }

        let elements = ((size - size_of::<Header>()) / size_of::<T>()) as u64;

        if header.size != elements {
            return Err(Error::WrongFileSize);
        }

        if header.typeid != get_hashed_type_id::<T>() {
            return Err(Error::WrongTypeId);   
        }

        Ok(PersistentArray {
            phantom_type: PhantomData,
            map: map,
            elements: elements,
        })
    }
}

impl<T: TypeBounds> Deref for PersistentArray<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe {
            slice::from_raw_parts(self.map.ptr().offset(size_of::<Header>() as isize) as *const T,
                                  self.elements as usize)
        }
    }
}

impl<T: TypeBounds> DerefMut for PersistentArray<T> {

    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            slice::from_raw_parts_mut(self.map.mut_ptr().offset(size_of::<Header>() as isize) as *mut T,
                                      self.elements as usize)
        }
    }
}