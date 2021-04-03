use std::collections::BTreeMap;

const KEYLEN: usize = 128;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Key(String);

impl Key {
    // creating keys longer than KEYLEN will result in an error
    // this is how we can guarantee "constant time" get() and put()
    fn new<S: Into<String>>(s: S) -> Result<Self, &'static str> {
        let s = s.into();
        if s.len() > KEYLEN {
            return Err("Key is too big");
        }
        Ok(Self(s))
    }
}

fn main() {
    // insert 1M entries
    let mut map = BTreeMap::new();
    for i in 0..1_000_000 {
        let k = Key::new(i.to_string()).unwrap();
        let v = i;
        map.insert(k, v);
    }

    // request one
    let k = Key::new("42").unwrap();
    let result = map.get(&k).unwrap();
    assert_eq!(*result, 42);
}
