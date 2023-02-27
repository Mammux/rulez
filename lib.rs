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

struct RuleMaps<'a> {
    pub entries: FrozenMap<String, Box<Entry>>,
    pub fields: FrozenMap<String, Box<Field<'a>>>,
}

impl RuleMaps<'_> {
    fn new() -> RuleMaps<'static> {
	RuleMaps { entries: FrozenMap::new(), fields: FrozenMap::new() }
    }
}

fn main() {
    let mut maps = RuleMaps::new();
    
    maps.entries.insert("test".to_string(), Box::new(Entry("e1".to_string())));
    maps.fields.insert("test".to_string(), Box::new(Field{id: "f1".to_string(), entry: maps.entries.get("test").unwrap()}));

    let _t = maps.entries.get("test");

    println!("Entry: {:?}", maps.entries["test"]);
    println!("Field: {:?}", maps.fields["test"]);
}

