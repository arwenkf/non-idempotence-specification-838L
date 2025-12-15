fn update(x:& mut u16) -> () {
    *x = 5;       
    *x = 3;
}

fn main() {
    let mut x = 0;
    update(&mut x);
}