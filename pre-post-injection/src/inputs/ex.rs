fn update(x:& mut u16, rb:& mut i16) -> () {
    *rb += 1; 
    *x = 5;       
    *x = 3
}

fn main() {
    let mut x = 0;
    let mut rb: i16 = -1;
    update(&mut x, &mut rb);
}