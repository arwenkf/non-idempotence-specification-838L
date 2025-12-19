//atomic start
// #[nids(z)]
fn test(z:& mut u16) -> () {
let mut exec_num = 1; 
 'label1: loop {
    *z += 20;
if (exec_num == 0) {
                    exec_num+=1;
                    continue 'label1;
                }
break; 
 }
}

//atomic start
// #[nids(z)]
fn main() {
let mut exec_num = 1; 
 'label2: loop {
    let mut z = 0;
    let mut x = 12;
    x += 1;

    let y = true;

    test(&mut z);

    if y {
        x+= 2;
    }
break; 
 }
}



