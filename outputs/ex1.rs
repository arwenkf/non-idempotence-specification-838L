#![allow(warnings)]

use std::collections::HashMap;
use once_cell::sync::Lazy;

trait SmartEq<T> {
    fn smart_eq(&self, other: T) -> bool;
}

impl<T: PartialEq + Copy> SmartEq<T> for T {
    fn smart_eq(&self, other: T) -> bool {
        *self == other
    }
}

static NID_TRACK: Lazy<HashMap<&'static str, Vec<Option<(i32, i32)>>>> =
    Lazy::new(|| {
        let mut m = HashMap::new();
        m.insert(
            "x",
            vec![None, Some((0, 13)), Some((0, 13)), Some((5, 4)), Some((5, 6)), Some((3, 6))],
        );
        m.insert(
            "rb",
            vec![None, Some((0, 12)), Some((0, 12)), Some((0, 12)), Some((1, 5)), Some((1, 5))],
        );
        m
    });


fn lookup(var: &str, exec_num: i32) -> Option<i32> {
    NID_TRACK
        .get(var)
        .and_then(|v| v.get(exec_num as usize)) 
        .and_then(|opt| *opt)
        .map(|(val, _)| val)
}

//atomic start
//#[nids(x,rb)]
fn update(x:& mut i32, rb:& mut i32, exec_num: &mut i32) -> () {

    'label1: loop {
        if *exec_num == 2 {*x =  0}  //restored from mem
        if *exec_num == 3 {*x =  5}  //restored from mem
        if *exec_num == 4 {*x =  5}  //restored from mem
        if *exec_num == 5 {*x =  3}  //restored from mem
        if *exec_num == 3 {*rb =  0} //restored from mem
        if *exec_num == 4 {*rb =  1} //restored from mem
        if *exec_num == 5 {*rb =  1} //restored from mem
        

        (if !(lookup("x", *exec_num).is_none() || x.smart_eq(lookup("x", *exec_num).unwrap())) { panic!("Pre-condition failed"); });
        *x = 5;   
        (if !x.smart_eq(5) { panic!("Post-condition failed"); });

        if (*exec_num == 1) {
            *exec_num += 1;
            continue 'label1;
        }

        (if !(lookup("rb", *exec_num).is_none() || rb.smart_eq(lookup("rb", *exec_num).unwrap())) { panic!("Pre-condition failed"); });
       
        *rb = 1;    
        (if !rb.smart_eq(1) { panic!("Post-condition failed"); });
       

        if (*exec_num == 2) {
            *exec_num += 1;
            continue 'label1;
        }
       
        (if !(lookup("x", *exec_num).is_none() || x.smart_eq(lookup("x", *exec_num).unwrap())) { panic!("Pre-condition failed"); });
        *x = 3;
        (if !x.smart_eq(3) { panic!("Post-condition failed"); });

        if (*exec_num == 3) {
            *exec_num+=1;
            continue 'label1;
        }
        break; 
    }
}

//atomic start
//#[nids(x,rb)]
fn main() {
    let mut exec_num = 1;
    let mut rb = 0;
    let mut x = 0;

    'label2: loop {
        (if !(lookup("rb", exec_num).is_none() || rb.smart_eq(lookup("rb", exec_num).unwrap())) { panic!("Pre-condition failed"); });
        rb = 0;
        (if !rb.smart_eq(0) { panic!("Post-condition failed"); });

        (if !(lookup("x", exec_num).is_none() || x.smart_eq(lookup("x", exec_num).unwrap())) { panic!("Pre-condition failed"); });     
        x = 0;
        (if !x.smart_eq(0) { panic!("Post-condition failed"); });
        update(&mut x, &mut rb, &mut exec_num);
        break; 
    }
}
