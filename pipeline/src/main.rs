use std::{collections::HashMap, env};


// TODO
//   eval write-up
//   make data structure correct
//   integrate ast with simul
//   insert crashes and make running tests
//   write up the simul part

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let m = ast_analyzer::run(filename, false);
    println!("{:?}", m);

    // for temp testing purposes: [ Some(0, 9),  Some(5, 2), Some(3, 3)]
    let mut nid_track: HashMap<String, Vec<(i32, i32)>> = HashMap::new();
    nid_track.insert("x".to_string(), vec![(0,13), (5,4), (3,6)]);
    nid_track.insert("rb".to_string(), vec![(0,12), (1,5)]);
    let x = aca::create_simul(filename, nid_track).unwrap();
    let _ = pre_post_injection::run(&x, "output.rs".to_string());
}
