#![feature(plugin)]
#![feature(test)]

#![plugin(clippy)]

#![warn(cast_possible_truncation, cast_possible_wrap, cast_precision_loss, cast_sign_loss,
        non_ascii_literal, shadow_same, string_add, string_add_assign, unicode_not_nfc)]

extern crate persistent_array;
extern crate test;

use persistent_array::PersistentArray;
use std::mem::size_of;
use test::Bencher;

const SIZE: u64 = 2*1024*1024;

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
