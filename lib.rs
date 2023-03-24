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

impl<'a> RuleMaps<'_> {
    fn new() -> RuleMaps<'a> {
	RuleMaps { entries: FrozenMap::new(), fields: FrozenMap::new() }
    }
    
    fn add_entry(&self, entry: Entry) {
	self.entries.insert(entry.0.clone(), Box::new(entry));
    }

}

fn main() {
    let maps = RuleMaps::new();
    
    maps.entries.insert("test_entry".to_string(), Box::new(Entry("e1".to_string())));
    maps.fields.insert("test_field".to_string(), Box::new(Field{id: "f1".to_string(), entry: maps.entries.get("test_entry").unwrap()}));

    let _t = maps.entries.get("test");

    println!("Entry: {:?}", maps.entries["test"]);
    println!("Field: {:?}", maps.fields["test"]);
}

