
//atomic start
//#[nids(z)]
fn test(z:& mut i32) -> () {
    *z = 20;
}

//atomic start
//#[nids(z)]
fn meow(z:&mut i32) -> () {
    *z = 1;
}

//atomic start
//#[nids(z)]
fn main() {
    let mut z = 0;
    let mut x = 12;
    x += 1;

    test(&mut z);

}


