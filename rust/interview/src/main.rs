fn main() {
    
}

#[derive(Default)]
struct MyMap {
    pairs: Vec<(String, u8)>
}

impl MyMap {
    fn assoc(&mut self, key: &str, value: u8) {
        // assume that the key is new
        self.pairs.push((String::from(key), value));
    }

    fn get(&self, key: &str) -> Option<u8> {
        for (k, v) in &self.pairs {
            if k == key {
                return Some(*v);
            }
        }
        None
    }
}

#[test]
fn test_mymap() {
    let mut mymap = MyMap::default();
    // TODO test empty mymap
    mymap.assoc("A", 1);
    mymap.assoc("B", 2);
    assert_eq!(mymap.get("A"), Some(1));
    assert_eq!(mymap.get("C"), None);
    // TODO replace key's value
}