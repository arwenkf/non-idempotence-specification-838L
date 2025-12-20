
//atomic start
//#[nids(z)]
fn test(z:& mut i32, exec_num: &mut i32) -> () {
 
 'label1: loop {
if exec_num == 2 {*z =  0} //restored from mem
if exec_num == 3 {*z =  20} //restored from mem
if exec_num == 4 {*z =  1} //restored from mem
    *z = 20;
if (exec_num == 1) {
                    exec_num+=1;
                    continue 'label1;
                }
break; 
 }
}

//atomic start
//#[nids(z)]
fn meow(z:&mut i32, exec_num: &mut i32) -> () {
 
 'label2: loop {
if exec_num == 2 {*z =  0} //restored from mem
if exec_num == 3 {*z =  20} //restored from mem
if exec_num == 4 {*z =  1} //restored from mem
    *z = 1;
if (exec_num == 2) {
                    exec_num+=1;
                    continue 'label2;
                }
break; 
 }
}

//atomic start
//#[nids(z)]
fn main() {
let mut exec_num = 1;
 
 'label3: loop {
    let mut z = 0;
    let mut x = 12;
    x += 1;

test(&mut z, &mut exec_num)

break; 
 }
}


