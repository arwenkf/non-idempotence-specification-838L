//atomic start
//#[nids(x,rb)]
fn update(x:& mut u16, rb:& mut u16) -> () {
let mut exec_num = 1; 
 'label1: loop {
if exec_num == 3 {*rb =  0} //restored from mem
if exec_num == 4 {*rb =  1} //restored from mem
if exec_num == 5 {*rb =  1} //restored from mem
if exec_num == 2 {*x =  0} //restored from mem
if exec_num == 3 {*x =  5} //restored from mem
if exec_num == 4 {*x =  5} //restored from mem
if exec_num == 5 {*x =  3} //restored from mem
   *x = 5;   
if (exec_num == 1) {
                    exec_num+=1;
                    continue 'label1;
                }
   *rb = 1;    
if (exec_num == 2) {
                    exec_num+=1;
                    continue 'label1;
                }
   *x = 3;
if (exec_num == 3) {
                    exec_num+=1;
                    continue 'label1;
                }
break; 
 }
}

//atomic start
//#[nids(x,rb)]
fn main() {
let mut exec_num = 1; 
 'label2: loop {
   let mut rb = 0;
   let mut x = 0;
   update(&mut x, &mut rb);
break; 
 }
}
