extern crate memmap;

pub mod persistent_array;

#[cfg(test)]
use std::default::Default;

#[cfg(test)]
#[derive(Debug, Copy, Clone)]
struct Pair {
    a: u32,
    b: u32,
}

#[cfg(test)]
impl Default for Pair {
    fn default() -> Pair {
        Pair {
            a: 0xddaaddaa,
            b: 0xffeeffee,
        }
    }
}

#[test]
fn test() {
    use persistent_array::PersistentArray;

    {
        let mut db: PersistentArray<Pair> = PersistentArray::new("pair.db", 1024).unwrap();

        db[0] = Pair { a: 1, b: 2 };

        assert_eq!(db[0].a, 1);
        assert_eq!(db[0].b, 2);

        assert_eq!(db[1023].a, 0xddaaddaa);
        assert_eq!(db[1023].b, 0xffeeffee);
    }
    {
        let db: PersistentArray<Pair> = PersistentArray::open("pair.db").unwrap();

        assert_eq!(db.len(), 1024);

        assert_eq!(db[0].a, 1);
        assert_eq!(db[0].b, 2);

        assert_eq!(db[1023].a, 0xddaaddaa);
        assert_eq!(db[1023].b, 0xffeeffee);
    }
}
