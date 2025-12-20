// #[atomic]
// #[nids(x)]
fn update1(x: &mut i32, exec_num: &mut i32) {
 
 'label1: loop {
if *exec_num == 2 {*x =  0} //restored from mem
if *exec_num == 3 {*x =  10} //restored from mem
if *exec_num == 4 {*x =  0} //restored from mem

lookup(&mut exec_num)
        *x = 10;
if (exec_num == 1) {
                    *exec_num+=1;
                    continue 'label1;
                }
        // post = 10

break; 
 }
}

// #[atomic]
// #[nids(x)]
fn update2(x: &i32, exec_num: &mut i32) {
 
 'label2: loop {
if *exec_num == 2 {*x =  0} //restored from mem
if *exec_num == 3 {*x =  10} //restored from mem
if *exec_num == 4 {*x =  0} //restored from mem

        let y: i32 = 14;

        if *x > y {
            println!("greater");
        }

break; 
 }
}

// #[atomic]
// #[nids(x)]
fn update3(x: &i32, exec_num: &mut i32) {
 
 'label3: loop {
if *exec_num == 2 {*x =  0} //restored from mem
if *exec_num == 3 {*x =  10} //restored from mem
if *exec_num == 4 {*x =  0} //restored from mem

        let y = *x + 5;

break; 
 }
}

// #[atomic]
// #[nids(x)]
fn update4(x: &mut i32, exec_num: &mut i32) {
 
 'label4: loop {
if *exec_num == 2 {*x =  0} //restored from mem
if *exec_num == 3 {*x =  10} //restored from mem
if *exec_num == 4 {*x =  0} //restored from mem
        
        *x = 0;
if (exec_num == 2) {
                    *exec_num+=1;
                    continue 'label4;
                }
        // post = 0
break; 
 }
}

// #[atomic]
// #[nids(x)]
fn update5(x: & i32, exec_num: &mut i32) {
 
 'label5: loop {
if *exec_num == 2 {*x =  0} //restored from mem
if *exec_num == 3 {*x =  10} //restored from mem
if *exec_num == 4 {*x =  0} //restored from mem
    let mut exec_num = 1; 

        y = 13;

        y = 14;

break; 
 }
}

// #[atomic]
// #[nids(x)]
fn update6(x: &mut i32, exec_num: &mut i32) {
 
 'label6: loop {
if *exec_num == 2 {*x =  0} //restored from mem
if *exec_num == 3 {*x =  10} //restored from mem
if *exec_num == 4 {*x =  0} //restored from mem

        let y = 13;

        let a = 10;

        let b = 20;

        let c = a + b;

break; 
 }
}

// #[atomic]
// #[nids(x)]
fn update7(x: &mut i32, exec_num: &mut i32) {
 
 'label7: loop {
if *exec_num == 2 {*x =  0} //restored from mem
if *exec_num == 3 {*x =  10} //restored from mem
if *exec_num == 4 {*x =  0} //restored from mem

        let a = 10;

        let b = 20;

break; 
 }
}

//#[atomic]
//#[nids(x)]
fn update8(x: &mut i32, exec_num: &mut i32) {
 
 'label8: loop {
if *exec_num == 2 {*x =  0} //restored from mem
if *exec_num == 3 {*x =  10} //restored from mem
if *exec_num == 4 {*x =  0} //restored from mem

        let a = 10;

        let b = 20;

        let mut y:i32 = 12;

        y = 13;

        y = 14;

break; 
 }
}

//#[atomic]
//#[nids(x)]
fn main() { // these ones are a little excessive lol
let mut exec_num = 1;
 
 'label9: loop {

        // pre = true
        let mut x: i32 = 0;
        // post = 0


update1(&mut x, &mut exec_num)

update2(&mut x, &mut exec_num)

update3(&mut x, &mut exec_num)

update4(&mut x, &mut exec_num)

update5(&mut x, &mut exec_num)

update6(&mut x, &mut exec_num)

update7(&mut x, &mut exec_num)

update8(&mut x, &mut exec_num)

break; 
 }
}
