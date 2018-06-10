// Copyright (c) 2016 Jonathan Nilsson
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#![feature(test)]

extern crate persistent_array;
extern crate test;

use persistent_array::PersistentArray;
use std::mem::size_of;
use test::Bencher;

const SIZE: u64 = 2 * 1024 * 1024;

#[derive(Debug, Default, Copy, Clone)]
struct Data {
    data: u64,
}

#[bench]
fn bench(b: &mut Bencher) {
    PersistentArray::<Data>::new("bench.db", SIZE).unwrap();

    b.bytes = SIZE * size_of::<Data>() as u64;
    b.iter(|| {
        let mut db = PersistentArray::<Data>::open("bench.db").unwrap();
        for i in 0..SIZE {
            db[i as usize].data = i as u64;
        }
    });
}
