
//atomic start
fn main() {
    let mut z = 0;
    let mut x = 12;
    x += 1;

    let y = true;

    test(z);

    if y {
        x+= 2;
    }
}

//atomic start
//nids(z)
fn test(z:& mut u16) -> () {
    z += 20;
}

