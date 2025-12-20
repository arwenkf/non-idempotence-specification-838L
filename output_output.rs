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
//#[nids(z)]
fn test(z:& mut i32, exec_num: &mut i32) -> () {
 
 'label1: loop {
if exec_num == 2 {*z =  0} //restored from mem
if exec_num == 3 {*z =  20} //restored from mem
if exec_num == 4 {*z =  1} //restored from mem
    (if !(lookup("z", exec_num).is_none() || z.smart_eq(lookup("z", exec_num).unwrap())) { panic!("Pre-condition failed"); });
    *z = 20;
    (if !z.smart_eq(20) { panic!("Post-condition failed"); });
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
    (if !(lookup("z", exec_num).is_none() || z.smart_eq(lookup("z", exec_num).unwrap())) { panic!("Pre-condition failed"); });
    *z = 1;
    (if !z.smart_eq(1) { panic!("Post-condition failed"); });
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
    (if !(lookup("z", exec_num).is_none() || z.smart_eq(lookup("z", exec_num).unwrap())) { panic!("Pre-condition failed"); });
    let mut z = 0;
    (if !z.smart_eq(0) { panic!("Post-condition failed"); });
    let mut x = 12;
    x += 1;

test(&mut z, &mut exec_num)

break; 
 }
}


