trait SmartEq<T> {
    fn smart_eq(&self, other: T) -> bool;
}

impl<T: PartialEq + Copy> SmartEq<T> for T {
    fn smart_eq(&self, other: T) -> bool {
        *self == other
    }
}

fn lookup(
    nid_track: &HashMap<&str, Vec<Option<(i32, usize)>>>, 
    var_name: &str, 
    exec_num: usize
) -> Option<i32> {
    nid_track.get(var_name)            
        .and_then(|v| v.get(exec_num))  
        .and_then(|opt| *opt)           
        .map(|(val, _)| val)           
}

//atomic start
//#[nids(x,rb)]
fn update(x:& mut u16, rb:& mut u16) -> () {
if !(lookup(x, exec_num).is_none() || x.smart_eq(lookup(x, exec_num).unwrap())) { panic!("Pre-condition failed"); }
let mut exec_num = 1; 
if !x.smart_eq(18) { panic!("Post-condition failed"); }
 if !(lookup(rb, exec_num).is_none() || rb.smart_eq(lookup(rb, exec_num).unwrap())) { panic!("Pre-condition failed"); }
 if !(lookup(x, exec_num).is_none() || x.smart_eq(lookup(x, exec_num).unwrap())) { panic!("Pre-condition failed"); }
 'label1: loop {
 if !rb.smart_eq(1) { panic!("Post-condition failed"); }
 if !x.smart_eq(18) { panic!("Post-condition failed"); }
if !(lookup(rb, exec_num).is_none() || rb.smart_eq(lookup(rb, exec_num).unwrap())) { panic!("Pre-condition failed"); }
if !(lookup(x, exec_num).is_none() || x.smart_eq(lookup(x, exec_num).unwrap())) { panic!("Pre-condition failed"); }
if exec_num == 1 {*x =  0} //restored from mem
if !rb.smart_eq(1) { panic!("Post-condition failed"); }
if !x.smart_eq(3) { panic!("Post-condition failed"); }
if exec_num == 2 {*x =  5} //restored from mem
if exec_num == 3 {*x =  5} //restored from mem
if exec_num == 4 {*x =  3} //restored from mem
if exec_num == 2 {*rb =  0} //restored from mem
if exec_num == 3 {*rb =  1} //restored from mem
if !(lookup(rb, exec_num).is_none() || rb.smart_eq(lookup(rb, exec_num).unwrap())) { panic!("Pre-condition failed"); }
if exec_num == 4 {*rb =  1} //restored from mem
if !rb.smart_eq(0) { panic!("Post-condition failed"); }
   if !(lookup(x, exec_num).is_none() || x.smart_eq(lookup(x, exec_num).unwrap())) { panic!("Pre-condition failed"); }
   *x = 5;   
   if !x.smart_eq(0) { panic!("Post-condition failed"); }
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
