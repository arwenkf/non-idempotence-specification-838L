// #[atomic]
// #[nids(x, y, z)]
fn update(x: &mut i32, y: &mut i32, z: &mut i32) -> () {
    *x = 5;
    *y = 3;
    *x = 1;
    *z = 13;
    *y = 1;

}

// #[atomic]
// #[nids(a, b)]
fn update_two(a: &mut i32, b: &mut i32) -> () {
    *a = 0;
    *a = 1;   
    *b = 0;
    *b = 1;   
    *a = 1; 
}

// #[atomic]
// #[nids(x, y, z, a, b, c, d)]
fn main() {
    let mut x = 0;
    let mut y = 1;
    let mut z = 2;
    let mut a = 3;
    let mut b = 4;
    let mut c = 5;
    let mut d = 6;

    update(&mut x, &mut y, &mut z);
    update_two(&mut a, &mut b);
    update_two(&mut c, &mut d);
}