use std::collections::HashMap;
// use std::cmp::max;
// use evalexpr::Value;

/*  An Entry instance represents the possibility of Instances in the store with type Entry.id.
    It is assumed that there will be at least Entry{"rule"} and Entry{"user"}. 
*/

#[allow(dead_code)]
#[derive(Debug,Clone)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Entry { 
    id: String
}

/*  A Field instance represents the possibility of a field being set for Instances of type
    Field.entry. 
*/

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Field {
    entry: String,
    id: String
}

/*  An Instance is the main type of object in the store.
*/

#[allow(dead_code)]
#[derive(Debug,Clone)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Instance {
    entry: String,
    id: String
}

/*  An Assignment is type of potential change, on a particular Field on a 
    particular Entry. It can later be used to create an Action that involves
    an Instance and can be checked vs. rules.
*/

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Assignment {
    id: String,
    field: String,
    value: String
}

/*  An Action is an Assignment on a particular Instance at a particular time
    that is either "proposed" (not checked vs rules yet) or has actually become 
    part of the Timeline in the store.
*/

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Action {
    timestamp: u64, // important that this is first for PartialOrd and Ord
    instance: String,
    assignment: String
}

/*  A Timeline is the list of timestampted Actions that have been accepted for one
    Instance in an object store. */

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Timeline {
    instance: String,
    actions: [String]
}


#[allow(dead_code)]
#[derive(Debug)]
struct ObjectStore {
    entries: Vec<Box<Entry>>,
    fields: Vec<Box<Field>>,
    instances: Vec<Box<Instance>>,
    assignments: Vec<Box<Assignment>>,
    actions: Vec<Box<Action>>,
    timelines: Vec<Box<Timeline>>
}

impl ObjectStore {
    fn new() -> ObjectStore {
        ObjectStore{ entries: vec!(), fields: vec!(), instances: vec!(), assignments: vec!(), actions: vec!(), timelines: vec!() }
    }

    fn add_entry(&mut self, entry: Entry) {
        let e = Box::new(entry);
        self.entries.push(e);
    }

    fn add_field(&mut self, field: Field) {
        let f = Box::new(field);
        self.fields.push(f);
    }

/*
    fn get_entry(self, id: String) -> & 'static Entry {
        let b = self.entries.iter().find( |&x| *x.id == id);
        return &((b.unwrap().to_owned()).clone());
    }

    fn get_field(self, id: String) -> & 'static Field {
        let b = self.fields.iter().find( |&&x| *x.id == id);
        return b.unwrap().clone();
    }
    */
}

/*  A State is a read-only image of the field values of an instance at a particular time. */

#[allow(dead_code)]
#[derive(Debug)]
struct State {
    instance: String,
    values: HashMap<String,String>,
    timestamp: u64
}

/*  This convenience function gets the state of an Instance based on its Timeline */

/* 
#[allow(dead_code)]
fn get_state(timeline: &Timeline, point: u64) -> State {
    let mut hm = HashMap::new();
    let mut stamp = 0;
    let mut acts = timeline.actions.to_vec(); // goes on the heap
    acts.sort(); // sorts based on timestamp
    for a in acts {
	if a.timestamp <= point && allowed(a.instance, &timeline, point, a.assignment) {
	    stamp = max(stamp, a.timestamp);
	    hm.insert(a.assignment.field.id.clone(), a.assignment.value.clone());
	}
    }
    let st = State{ instance: timeline.instance.id.clone(),
		    values: hm,
		    timestamp: stamp}; 
    return st;
}
*/

/*  Stub for a method that verifies an Assignment using a particular rule.
 */

#[allow(dead_code)]
fn allowed(instance: &Instance, _timeline: &Timeline, _point: u64, _assignment: &Assignment) -> bool {
    // TODO: find all "rule" instances in the object store, check each one
    // OR: find the root "rule" and start evaluating
    if instance.entry != "rule" { return true; }
    // let st = get_state(timeline, point);

    return false;
}

/*  Creates a new Instance */

fn _mk_instance(entry: &Entry, id: String) -> Instance {
    return Instance{ id: id, entry: entry.id.to_string() };
}

fn mk_object_store<'a>() -> ObjectStore{
    let mut os = ObjectStore::new();

    os.add_entry(Entry{ id: "rule".to_string() });
    os.add_field(Field{ id: "criteria".to_string(), entry: "rule".to_owned()});
    os.add_field(Field{ id: "author".to_string(), entry: "rule".to_owned()});

    // os.entries.push(Box::new(Entry{ id: "rule".to_string() })); // magic entry id for rules
    // os.entries.push(Box::new(Entry{ id: "user".to_string() })); // magic entry id for users (actual people)
    // os.fields.push(Box::new(Field{ entry: &v, id: "name".to_string() }));

    os
}

    /*
    let v = Entry{ id: "rule".to_string() }; // magic entry id for rules
    let u = Entry{ id: "user".to_string() }; // magic entry id for users (actual people)
    let c = Field{ entry: &v, id: "name".to_string() };
    let a = Assignment{ field: &c, value: "Ground rule".to_string() };
    let a2 = Assignment{ field: &c, value: "First rule".to_string() };
    let i = Box::new(Instance{ entry: &v, id: "rule-0".to_string() });
    let t = Action{ instance: &i, assignment: &a, timestamp: 1000 };
    let t2 = Action{ instance: &i, assignment: &a2, timestamp: 2000 };
    let ts = [ &t, &t2 ];
    let i2 = Box::new(mk_instance(&u, "bobby".to_string()));
    let tl = Timeline{ instance: &i,
		       actions: &ts };
    let tl2 = Timeline{ instance: &i2, actions: &[]};
    */

fn main() {

    // let tls = mk_test_timeline();

    // let state = get_state(tls.get(0).unwrap(), 1500);


    let os = mk_object_store();
    println!("{:?}", &os);

    /*
    println!("Entry: {:#?}", v);
    println!("Field: {:#?}", c);
    println!("Assignment: {:#?}", a);
    println!("Instance: {:#?}", i);
    println!("Action: {:#?}", t);
    println!("Timeline: {:#?}", tl); 
    */
}

#[cfg(test)]
mod tests {

    use evalexpr::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn eval_works() {
        assert_eq!(eval("1 + 2 + 3"), Ok(Value::from(6)));
    }
}

