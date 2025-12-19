use std::env;


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


    let _ = aca::create_simul(filename, m);
}
 