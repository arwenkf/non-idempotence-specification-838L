fn update(x:& mut u16, rb:& mut i16) -> () {
    /* << Pre-condition: rb == -1 >> */
    *rb += 1; 
    /* << Post-condition: rb == 1 >> */

    /* << Pre-condition: x == 0 >> */
    *x = 5;       
    /* << Post-condition: x == 5 >> */

    /* << Pre-condition: x == 5 >> */
    *x = 3
    /* << Post-condition: x == 3 >> */

}

fn main() {
    /* << Pre-condition: true >> */
    let mut x = 0;
    /* << Post-condition: x == 0 >> */

    /* << Pre-condition: true >> */
    let mut rb: i16 = -1;
    /* << Post-condition: rb == -1 >> */

    update(&mut x, &mut rb);
}
