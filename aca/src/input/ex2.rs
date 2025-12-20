//atomic start
//#[nids(x,rb)]
fn update(x:& mut u16, rb:& mut u16) -> () {
   *x = 5;   
   *rb = 1;    
   *x = 3;
}

//atomic start
//#[nids(x,rb)]
fn main() {
   let mut rb = 0;
   let mut x = 0;
   update(&mut x, &mut rb);
}