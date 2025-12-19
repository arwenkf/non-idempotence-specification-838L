#[atomic]
#[nids(x, rb)]
fn update(rb:& mut i32, x:& mut u16) -> () {
    *rb += 1 ;
    *x = 5;       
    *x = 3;
}

#[atomic]
#[nids(x, rb)]
fn main() {
    let mut rb = -1;
    let mut x = 0;
    update(&mut rb, &mut x);
}