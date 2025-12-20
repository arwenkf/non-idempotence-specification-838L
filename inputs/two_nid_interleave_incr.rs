// #[atomic]
// #[nids(x, rb)]
fn update(rb:& mut i32, x:& mut i32) -> () {
    *rb = 1;
    *x = 2;
    *rb = 3;
    *x = 4;
    *rb = 5;
    *x = 6;
    *rb = 7;
    *x = 8;
    *rb = 9;
    *x = 10;
    *rb = 11;
    *x = 12;
}

// #[atomic]
// #[nids(x, rb)]
fn main() {
    let mut rb = -1;
    let mut x = 1;
    update(&mut rb, &mut x);
}