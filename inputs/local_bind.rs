#[atomic]
#[nids(x)]
fn update(x:& mut u16) -> () {
    *x = 5;   
    let mut y = 14;    
    *x = 3;
    *y  = 12;
}

#[atomic]
#[nids(x)]
fn main() {
    let mut x = 0;
    update(&mut x);
}