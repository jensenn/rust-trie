const KEYLEN: usize = 128;

struct Map<V> {
    root: Node<V>,
}

impl<V> Map<V> {
    fn new() -> Self {
        Self {
            root: Node::<V>::default(),
        }
    }

    fn put(&mut self, k: &Key, v: V) {
        let mut cur_node = &mut self.root;
        for idx in k.0.bytes() {
            cur_node = cur_node.get_or_add_child(idx);
        }
        (*cur_node).value = Some(Box::new(v));
    }

    fn get<'a>(&'a self, k: &Key) -> Option<&'a V> {
        let mut cur_node = &self.root;
        for idx in k.0.bytes() {
            match cur_node.get_child(idx) {
                Some(child) => {
                    cur_node = child;
                }
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
    // in Rust, Option<Box>> is a nullable pointer
    // A node may or may not hold a value
    value: Option<Box<V>>,
    // Children is an array of 256 pointers to other nodes
    // A node may or may not have children, saving allocation on leaf nodes
    children: Option<Box<[Option<Box<Node<V>>>; 256]>>,
}

impl<V> Default for Node<V> {
    fn default() -> Self {
        Self {
            value: None,
            children: None,
        }
    }
}

impl<V> Node<V> {
    fn get_child(&self, idx: u8) -> Option<&Node<V>> {
        if self.children.is_none() {
            return None;
        }
        let child = &self.children.as_ref().unwrap()[idx as usize];
        if child.is_none() {
            return None;
        }
        child.as_ref().map(|x| x.as_ref())
    }

    fn get_or_add_child(&mut self, idx: u8) -> &mut Node<V> {
        let ch = self.children.get_or_insert_with(new_child_array);
        ch[idx as usize].get_or_insert_with(|| {
            Box::new(Node {
                value: None,
                children: None,
            })
        })
    }
}

fn new_child_array<V>() -> Box<[Option<Box<Node<V>>>; 256]> {
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
    Box::new(children)
}

fn main() {
    const ITERATIONS: i32 = 50_000_000;

    // push
    let mut map = Map::new();
    for i in 0..ITERATIONS {
        if i % 10_000_000 == 0 {
            println!("{}", i);
        }
        let s = i.to_string();
        let k = Key::new(&s).unwrap();
        let v = i;
        map.put(&k, v);
    }

    // get
    for i in 0..ITERATIONS {
        if i % 10_000_000 == 0 {
            println!("{}", i);
        }
        let s = i.to_string();
        let k = Key::new(&s).unwrap();
        let result = map.get(&k).unwrap();
        assert_eq!(*result, i);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn key_too_big() {
        let bigstr = String::from_utf8(vec!['a' as u8; 129]).unwrap();
        let k = Key::new(&bigstr);
        assert!(k.is_err());
    }

    #[test]
    fn test_get_empty() {
        let map = Map::<i32>::new();

        let k = Key::new("red").unwrap();
        let result = map.get(&k);
        assert!(result.is_none());
    }

    #[test]
    fn test_put_empty_key() {
        let mut map = Map::new();

        let k = Key::new("").unwrap();
        let v = 42;
        map.put(&k, v);

        let result = map.get(&k).unwrap();
        assert_eq!(*result, 42);
    }

    #[test]
    fn test_put_one() {
        let mut map = Map::new();

        let k = Key::new("red").unwrap();
        let v = 42;
        map.put(&k, v);

        let result = map.get(&k).unwrap();
        assert_eq!(*result, 42);

        let result = map.get(&k).unwrap();
        assert_eq!(*result, 42);
    }

    #[test]
    fn test_put_two() {
        let mut map = Map::new();

        let k = Key::new("red").unwrap();
        let v = 42;
        map.put(&k, v);

        let k = Key::new("green").unwrap();
        let v = 21;
        map.put(&k, v);

        let k = Key::new("red").unwrap();
        let result = map.get(&k).unwrap();
        assert_eq!(*result, 42);

        let k = Key::new("blue").unwrap();
        let result = map.get(&k);
        assert!(result.is_none());
    }
}
