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
    println!("WHYYYYYY {}", filename);
    let m = ast_analyzer::run(filename, false);
    println!(":LAKSJD:LKJ {:?}", m);

    // for temp testing purposes: [ Some(0, 9),  Some(5, 2), Some(3, 3)]
    let mut nid_track: HashMap<String, Vec<(i32, i32)>> = HashMap::new();
    // nid_track.insert("z".to_string(), vec![(0,17), (20,5), (1,11)]); //ex1

    // nid_track.insert("x".to_string(), vec![(0,13), (5,4), (3,6)]); //ex2 part 1
    // nid_track.insert("rb".to_string(), vec![(0,12), (1,5)]); // ex2 part 2

    nid_track.insert("x".to_string(), vec![(0,95), (10, 6), (0,35)]);
    let x = aca::create_simul(filename, nid_track).unwrap();
    let _ = pre_post_injection::run(&x, "output.rs".to_string());
}
