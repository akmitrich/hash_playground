use hash_playground::{chain::ChainTable, HashTable};

fn main() {
    let mut table = ChainTable::new();
    println!("Empty: {:?}", table);
    table.insert("key".into(), 42);
    table.insert("Key".into(), 55);
    table.insert("abc".into(), 21);
    table.insert("cat".into(), 23);
    table.insert("dog".into(), 27);
    table.insert("oval".into(), 33);
    table.insert("key".into(), 41);
    table.insert("Cow".into(), 144);
    println!("Gonna resize: {:?}", table);
    table.insert("resize".into(), 2);
    println!("Insert complete: {:?}", table);
    table.remove("key".into());
    println!("Final: {:?}", table);
}
