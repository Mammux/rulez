use elsa::map::FrozenMap;

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Entry(String);

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Field<'a> {
    id: String,
    entry: &'a Entry
}

fn main() {
    let entry_map: FrozenMap<String, Box<Entry>> = FrozenMap::new();
    entry_map.insert("test".to_string(), Box::new(Entry("e1".to_string())));

    let field_map: FrozenMap<String, Box<Field>> = FrozenMap::new();
    field_map.insert("test".to_string(), Box::new(Field{id: "f1".to_string(), entry: entry_map.get("test").unwrap()}));

    let _t = entry_map.get("test");

    println!("Entry: {:?}", entry_map["test"]);
    println!("Field: {:?}", field_map["test"]);
}

