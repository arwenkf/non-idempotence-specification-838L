// #[atomic]
// #[nids(x)]
fn update1(x: &mut i32) {

        // pre = lookup()
        *x = 10;
        // post = 10

}

// #[atomic]
// #[nids(x)]
fn update2(x: &i32) {

        let y: i32 = 14;

        if *x > y {
            println!("greater");
        }

}

// #[atomic]
// #[nids(x)]
fn update3(x: &i32) {

        let y = *x + 5;

}

// #[atomic]
// #[nids(x)]
fn update4(x: &mut i32) {
        
        *x = 0;
        // post = 0
}

// #[atomic]
// #[nids(x)]
fn update5(x: & i32) {
    let mut exec_num = 1; 

        y = 13;

        y = 14;

}

// #[atomic]
// #[nids(x)]
fn update6(x: &mut i32) {

        let y = 13;

        let a = 10;

        let b = 20;

        let c = a + b;

}

// #[atomic]
// #[nids(x)]
fn update7(x: &mut i32) {

        let a = 10;

        let b = 20;

}

//#[atomic]
//#[nids(x)]
fn update8(x: &mut i32) {

        let a = 10;

        let b = 20;

        let mut y:i32 = 12;

        y = 13;

        y = 14;

}

//#[atomic]
//#[nids(x)]
fn main() { // these ones are a little excessive lol

        // pre = true
        let mut x: i32 = 0;
        // post = 0


        update1(&mut x);

        update2(&mut x);

        update3(&mut x);

        update4(&mut x);

        update5(&mut x);

        update6(&mut x);

        update7(&mut x);

        update8(&mut x);

}