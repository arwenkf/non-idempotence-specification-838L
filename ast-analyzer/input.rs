#[atomic]
#[nids(x, rb)]
fn update(rb: &mut u32, x:& mut u16) -> () {
   *rb += 1;
   *x = 5;
   *x = 3;
}
#[rustc_main]
fn main() {
   let mut x = 0;
   let mut rb = 0;
   update(&mut rb, &mut x);
}