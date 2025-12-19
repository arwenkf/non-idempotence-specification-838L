#[atomic]
#[nids(x, rb)]
fn update(rb:& mut i32, x:& mut u16) -> () {
    *rb += 1;
    *x *= 2;
    *rb += 1;
    *x *= 2;
    *rb += 1;
    *x *= 2;
    *rb += 1;
    *x *= 2;
    *rb += 1;
    *x *= 2;
    *rb += 1;
    *x *= 2;
}

#[atomic]
#[nids(x, rb)]
fn main() {
    let mut rb = -1;
    let mut x = 1;
    update(&mut rb, &mut x);
}