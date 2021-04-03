use std::collections::HashMap;

const KEYLEN: usize = 128;

#[derive(Clone, PartialEq, Eq, Hash)]
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
    const ITERATIONS: i32 = 50_000_000;

    // push
    let mut map = HashMap::new();
    for i in 0..ITERATIONS {
        if i % 10_000_000 == 0 {
            println!("{}", i);
        }
        let k = Key::new(i.to_string()).unwrap();
        let v = i;
        map.insert(k, v);
    }

    // get
    for i in 0..ITERATIONS {
        if i % 10_000_000 == 0 {
            println!("{}", i);
        }
        let k = Key::new(i.to_string()).unwrap();
        let result = map.get(&k).unwrap();
        assert_eq!(*result, i);
    }
}
