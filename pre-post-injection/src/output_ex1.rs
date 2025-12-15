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

fn update(x:& mut u16) -> () {
    if !(lookup(x, exec_num).is_none() || x.smart_eq(lookup(x, exec_num).unwrap())) { panic!("Pre-condition failed"); }
    *x = 5;       
    if !x.smart_eq(5) { panic!("Post-condition failed"); }
    if !(lookup(x, exec_num).is_none() || x.smart_eq(lookup(x, exec_num).unwrap())) { panic!("Pre-condition failed"); }
    *x = 3;
    if !x.smart_eq(3) { panic!("Post-condition failed"); }
}

fn main() {
    if !(lookup(x, exec_num).is_none() || x.smart_eq(lookup(x, exec_num).unwrap())) { panic!("Pre-condition failed"); }
    let mut x = 0;
    if !x.smart_eq(0) { panic!("Post-condition failed"); }
    update(&mut x);
}
