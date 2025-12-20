// #[atomic]
// #[nids(y)]
fn update2(rb: &mut i32, y: &mut i32) {
    *rb = 2;
    *y = 230942;
}

// #[atomic]
// #[nids(x)]
fn update(rb: &mut i32, x: &mut i32) -> () {
    *rb = 1;
    *x = 5;
    *x = 3;

    update2(rb, x);
}

// #[atomic]
// #[nids(x)]
fn main() {
    let mut rb = -1;
    let mut x = 0;
   
    update(&mut rb, &mut x);
}