extern crate memmap;

mod persistent_array;

use persistent_array::PersistentArray;

#[derive(Copy, Clone, Default)]
struct Pair {
    a: i32,
    b: i32,
}

#[test]
fn test() {
    {
        let mut db: PersistentArray<Pair> = PersistentArray::new("pair.db", 1024).unwrap();

        /*db[0] = Pair { a: 1, b: 2 };

        assert_eq!(db[0].a, 1);
        assert_eq!(db[0].b, 2);

        assert_eq!(db[1023], 0);
        assert_eq!(db[1023], 0);*/
    }
    {
        let mut db: PersistentArray<Pair> = PersistentArray::open("pair.db").unwrap();

        /*assert_eq!(db.len(), 1024);

        assert_eq!(db[0].a, 1);
        assert_eq!(db[0].b, 2);

        assert_eq!(db[1023], 0);
        assert_eq!(db[1023], 0);*/
    }
}
