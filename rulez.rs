use std::collections::HashMap;
use std::cmp::max;

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Entry {
    id: String
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Field<'a> {
    entry: &'a Entry,
    id: String
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Instance<'a> {
    entry: &'a Entry,
    id: String
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Assignment<'a> {
    field: &'a Field<'a>,
    value: String
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Action<'a> {
    timestamp: u64, // important that this is first for PartialOrd and Ord
    instance: &'a Instance<'a>,
    assignment: &'a Assignment<'a>
}

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Timeline<'a> {
    instance: &'a Instance<'a>,
    actions: &'a [&'a Action<'a>]
}

#[allow(dead_code)]
#[derive(Debug)]
struct State {
    instance: String,
    values: HashMap<String,String>,
    timestamp: u64
}

#[allow(dead_code)]
fn get_state(timeline: Timeline, point: u64) -> State {
    let mut hm = HashMap::new();
    let mut stamp = 0;
    let mut acts = timeline.actions.to_vec(); // goes on the heap
    acts.sort(); // sorts based on timestamp
    for a in acts {
	if a.timestamp <= point && allowed(a.instance, a.assignment) {
	    stamp = max(stamp, a.timestamp);
	    hm.insert(a.assignment.field.id.clone(), a.assignment.value.clone());
	}
    }
    let st = State{ instance: timeline.instance.id.clone(),
		    values: hm,
		    timestamp: stamp}; 
    return st;
}

#[allow(dead_code)]
fn allowed(instance: &Instance, _assignment: &Assignment) -> bool {
    if instance.entry.id != "rule" { return true; }
    return false;
}

fn mk_instance(entry: &Entry, id: String) -> Instance {
    return Instance{ entry: entry, id: id };
}

fn main() {
    let v = Entry{ id: "rule".to_string() }; // magic entry id for rules
    let u = Entry{ id: "user".to_string() }; // magic entry id for users
    let c = Field{ entry: &v, id: "name".to_string() };
    let a = Assignment{ field: &c, value: "Ground rule".to_string() };
    let a2 = Assignment{ field: &c, value: "First rule".to_string() };
    let i = Instance{ entry: &v, id: "rule-0".to_string() };
    let t = Action{ instance: &i, assignment: &a, timestamp: 1000 };
    let t2 = Action{ instance: &i, assignment: &a2, timestamp: 2000 };
    let ts = [ &t, &t2 ];
    let tl = Timeline{ instance: &i,
		       actions: &ts };

    let state = get_state(tl, 1500);
    let _i2 = mk_instance(&u, "bobby".to_string());
    
    println!("{:?}", state);

    /*
    println!("Entry: {:#?}", v);
    println!("Field: {:#?}", c);
    println!("Assignment: {:#?}", a);
    println!("Instance: {:#?}", i);
    println!("Action: {:#?}", t);
    println!("Timeline: {:#?}", tl); 
    */
}

