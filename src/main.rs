const KEYLEN: usize = 128;

struct Map<V> {
    head: Node<V>,
}

impl<V> Map<V> {
    fn new() -> Self {
        Self {
            head: Node::<V>::default(),
        }
    }

    fn put(&mut self, k: Key, v: V) {
        let mut cur_node = &mut self.head;
        for c in k.0.chars() {
            let idx = c as usize;
            if cur_node.children[idx].is_none() {
                let new_node = Node::<V>::default();
                cur_node.children[idx] = Some(Box::new(new_node));
            }
            cur_node = cur_node.children[idx].as_mut().unwrap()
        }
        (*cur_node).value = Some(Box::new(v));
    }

    fn get<'a>(&'a self, k: &Key) -> Option<&'a V> {
        let mut cur_node = &self.head;
        for c in k.0.chars() {
            let idx = c as usize;
            match &cur_node.children[idx] {
                Some(node) => cur_node = node.as_ref(),
                None => return None,
            }
        }
        cur_node.value.as_ref().map(|value| value.as_ref())
    }
}

#[derive(Clone)]
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

struct Node<V> {
    value: Option<Box<V>>,
    children: [Option<Box<Node<V>>>; 256],
}

impl<V> Default for Node<V> {
    fn default() -> Self {
        // This is a sharp edge of Rust. This is how you create an array of NULL pointers
        let children = {
            let mut data: [std::mem::MaybeUninit<Option<Box<Node<V>>>>; 256] =
                unsafe { std::mem::MaybeUninit::uninit().assume_init() };
            for elem in &mut data[..] {
                unsafe {
                    std::ptr::write(elem.as_mut_ptr(), None);
                }
            }
            unsafe { std::mem::transmute::<_, [Option<Box<Node<V>>>; 256]>(data) }
        };
        Self {
            value: None,
            children,
        }
    }
}

fn main() {
    // insert 1M entries
    let mut map = Map::new();
    for i in 0..1_000_000 {
        let k = Key::new(i.to_string()).unwrap();
        let v = i;
        map.put(k, v);
    }

    // request one
    let k = Key::new("42").unwrap();
    let result = map.get(&k).unwrap();
    assert_eq!(*result, 42);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn key_too_big() {
        let bigstr = String::from_utf8(vec!['a' as u8; 129]).unwrap();
        let k = Key::new(bigstr);
        assert!(k.is_err());
    }

    #[test]
    fn test_put_empty_key() {
        let mut map = Map::new();

        let k = Key::new("").unwrap();
        let v = 42;
        map.put(k.clone(), v);

        let result = map.get(&k).unwrap();
        assert_eq!(*result, 42);
    }

    #[test]
    fn test_put_one() {
        let mut map = Map::new();

        let k = Key::new("red").unwrap();
        let v = 42;
        map.put(k.clone(), v);

        let result = map.get(&k).unwrap();
        assert_eq!(*result, 42);
    }

    #[test]
    fn test_put_two() {
        let mut map = Map::new();

        let k = Key::new("red").unwrap();
        let v = 42;
        map.put(k, v);

        let k = Key::new("green").unwrap();
        let v = 21;
        map.put(k, v);

        let k = Key::new("red").unwrap();
        let result = map.get(&k).unwrap();
        assert_eq!(*result, 42);
    }
}
