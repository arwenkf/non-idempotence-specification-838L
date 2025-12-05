fn update(x:& mut u16, rb:& mut i16) -> () {
    /* << Pre-condition: rb == -1 >> */
    if !(rb == -1) { panic!("Pre-condition failed: rb == -1"); }
    *rb += 1; 
    /* << Post-condition: rb == 1 >> */
    if !(rb == 1) { panic!("Post-condition failed: rb == 1"); }

    /* << Pre-condition: x == 0 >> */
    if !(x == 0) { panic!("Pre-condition failed: x == 0"); }
    *x = 5;       
    /* << Post-condition: x == 5 >> */
    if !(x == 5) { panic!("Post-condition failed: x == 5"); }

    /* << Pre-condition: x == 5 >> */
    if !(x == 5) { panic!("Pre-condition failed: x == 5"); }
    *x = 3
    /* << Post-condition: x == 3 >> */
    if !(x == 3) { panic!("Post-condition failed: x == 3"); }

}

fn main() {
    /* << Pre-condition: true >> */
    if !(true) { panic!("Pre-condition failed: true"); }
    let mut x = 0;
    /* << Post-condition: x == 0 >> */
    if !(x == 0) { panic!("Post-condition failed: x == 0"); }

    /* << Pre-condition: true >> */
    if !(true) { panic!("Pre-condition failed: true"); }
    let mut rb: i16 = -1;
    /* << Post-condition: rb == -1 >> */
    if !(rb == -1) { panic!("Post-condition failed: rb == -1"); }

    update(&mut x, &mut rb);
}
