//atomic start
// #[nids(z)]
fn test(z:& mut u16) -> () {
    *z += 20;
}

//atomic start
// #[nids(z)]
fn main() {
    let mut z = 0;
    let mut x = 12;
    x += 1;

    let y = true;

    test(&mut z);

    if y {
        x+= 2;
    }
}



