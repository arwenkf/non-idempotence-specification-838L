// #[atomic]
// #[nids(x)]
fn update(x:& mut i32) -> () {
    *x = 8;       
    *x = 3;
    *x = 8;
}

// #[atomic]
// #[nids(x)]
fn main() {
    let mut x = 0;
    update(&mut x);
}