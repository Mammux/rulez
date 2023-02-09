use std::collections::HashMap;
use std::cmp::max;
use evalexpr::Value;

/*  An Entry instance represents the possibility of Instances in the store with type Entry.id.
    It is assumed that there will be at least Entry{"rule"} and Entry{"user"}. 
*/

#[allow(dead_code)]
#[derive(Debug)]
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
struct Field<'a> {
    entry: &'a Entry,
    id: String
}

/*  An Instance is the main type of object in the store.
*/

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Instance<'a> {
    entry: &'a Entry,
    id: String
}

/*  An Assignment is type of potential change, on a particular Field on a 
    particular Entry. It can later be used to create an Action that involves
    an Instance and can be checked vs. rules.
*/

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Assignment<'a> {
    field: &'a Field<'a>,
    value: String
}

/*  An Action is an Assignment on a particular Instance at a particular time
    that is either "proposed" (not checked vs rules yet) or has actually become 
    part of the Timeline in the store.
*/

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Action<'a> {
    timestamp: u64, // important that this is first for PartialOrd and Ord
    instance: &'a Instance<'a>,
    assignment: &'a Assignment<'a>
}

/*  A Timeline is the list of timestampted Actions that have been accepted for one
    Instance in an object store. */

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Timeline<'a> {
    instance: &'a Instance<'a>,
    actions: &'a [&'a Action<'a>]
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

#[allow(dead_code)]
fn get_state(timeline: Timeline, point: u64) -> State {
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

/*  Stub for a method that verifies an Assignment using a particular rule.
 */

#[allow(dead_code)]
fn allowed(instance: &Instance, timeline: &Timeline, point: u64, assignment: &Assignment) -> bool {
    // TODO: find all "rule" instances in the object store, check each one
    // OR: find the root "rule" and start evaluating
    if instance.entry.id != "rule" { return true; }
    // let st = get_state(timeline, point);

    return false;
}

/*  Creates a new Instance */

fn mk_instance(entry: &Entry, id: String) -> Instance {
    return Instance{ entry: entry, id: id };
}

fn main() {
    let v = Entry{ id: "rule".to_string() }; // magic entry id for rules
    let u = Entry{ id: "user".to_string() }; // magic entry id for users (actual people)
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

