use std::collections::BTreeMap;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug)]
struct Datastore {
    inner: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl Datastore {
    fn new() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl Deref for Datastore {
    type Target = BTreeMap<Vec<u8>, Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        println!("[Debug] Datastore deref");
        &self.inner
    }
}

impl DerefMut for Datastore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        println!("[Debug] Datastore deref mut");
        &mut self.inner
    }
}

fn main() {
    println!("Building a new data store:");
    let mut ds = Datastore::new();

    // Call the method: 'len' provided by the BTreeMap
    println!("- Datastore len: {}", ds.len());
    println!("- Datastore content: {:?}", ds);

    println!("Now inserting some data");
    let k1 = vec![11, 22, 33];
    let v1 = vec![65, 66, 67];
    // Call the method: 'insert' provided by the BTreeMap
    ds.insert(k1, v1);

    println!("And finally, dump it:");
    println!("- Datastore len: {}", ds.len());
    println!("- Datastore content: {:?}", ds);
}
