#[atomic]
#[nids(x)]
fn update1(x: &mut i32) {
    let mut exec_num = 1; 
    'label1: loop {
        // pre = lookup()
        *x = 10;
        // post = 10
        if (exec_num == 1) {
            exec_num += 1;
            continue 'label1;
        }
        break;
    }
}

#[atomic]
#[nids(x)]
fn update2(x: &i32) {
    let mut exec_num = 1; 
    'label1: loop {
        let y: i32 = 14;
        if (exec_num == 1) {
            exec_num += 1;
            continue 'label1;
        }
        if x > y {
            println!("greater");
        }
        if (exec_num == 2) {
            exec_num += 1;
            continue 'label1;
        }
        break;
    }
}

#[atomic]
#[nids(x)]
fn update3(x: &i32) {
    let mut exec_num = 1; 
    'label1: loop {
        let y = *x + 5;
        if (exec_num == 1) {
            exec_num += 1;
            continue 'label1;
        }
        break;
    }
}

#[atomic]
#[nids(x)]
fn update4(x: &mut i32) {
    let mut exec_num = 1; 
    'label1: loop {
        // pre = lookup()
        *x = 0;
        // post = 0
        if (exec_num == 1) {
            exec_num += 1;
            continue 'label1;
        }
        break;
    }
}

#[atomic]
#[nids(x)]
fn update5(x: & i32) {
    let mut exec_num = 1; 
    'label1: loop {
        let mut y:i32 = 12;
        if (exec_num == 1) {
            exec_num += 1;
            continue 'label1;
        }
        y = 13;
         if (exec_num == 2) {
            exec_num += 1;
            continue 'label1;
        }
        y = 14;
         if (exec_num == 3) {
            exec_num += 1;
            continue 'label1;
        }

        break;
    }
}

#[atomic]
#[nids(x)]
fn update6(x: &mut i32) {
    let mut exec_num = 1; 
    'label1: loop {
        let y = 13;
        if (exec_num == 1) {
            exec_num += 1;
            continue 'label1;
        }
        let a = 10;
        if (exec_num == 2) {
            exec_num += 1;
            continue 'label1;
        }
        let b = 20;
        if (exec_num == 3) {
            exec_num += 1;
            continue 'label1;
        }
        let c = a + b;
        if (exec_num == 4) {
            exec_num += 1;
            continue 'label1;
        }
        break;
    }
}

#[atomic]
#[nids(x)]
fn update7(x: &mut i32) {
    let mut exec_num = 1; 
    'label1: loop {
        let a = 10;
        if (exec_num == 1) {
            exec_num += 1;
            continue 'label1;
        }
        let b = 20;
        if (exec_num == 2) {
            exec_num += 1;
            continue 'label1;
        }
        break;
    }
}

#[atomic]
#[nids(x)]
fn update8(x: &mut i32) {
    let mut exec_num = 1; 
    'label1: loop {
        let a = 10;
        if (exec_num == 1) {
            exec_num += 1;
            continue 'label1;
        }
        let b = 20;
        if (exec_num == 2) {
            exec_num += 1;
            continue 'label1;
        }
        let mut y:i32 = 12;
        if (exec_num == 3) {
            exec_num += 1;
            continue 'label1;
        }
        y = 13;
        if (exec_num == 4) {
            exec_num += 1;
            continue 'label1;
        }
        y = 14;
        if (exec_num == 5) {
            exec_num += 1;
            continue 'label1;
        }
        break;
    }
}

#[atomic]
#[nids(x)]
fn main() { // these ones are a little excessive lol
    let mut exec_num = 1; 
    'label1: loop {
        // pre = true

        let mut x: i32 = 0;
        // post = 0
        if (exec_num == 1) {
            exec_num += 1;
            continue 'label1;
        }

        update1(&mut x);
        if (exec_num == 2) {
            exec_num += 1;
            continue 'label1;
        }
        update2(&mut x);
        if (exec_num == 3) {
            exec_num += 1;
            continue 'label1;
        }
        update3(&mut x);
        if (exec_num == 4) {
            exec_num += 1;
            continue 'label1;
        }
        update4(&mut x);
        if (exec_num == 5) {
            exec_num += 1;
            continue 'label1;
        }
        update5(&mut x);
        if (exec_num == 6) {
            exec_num += 1;
            continue 'label1;
        }
        update6(&mut x);
        if (exec_num == 7) {
            exec_num += 1;
            continue 'label1;
        }
        update7(&mut x);
        if (exec_num == 8) {
            exec_num += 1;
            continue 'label1;
        }
        update8(&mut x);
        if (exec_num == 9) {
            exec_num += 1;
            continue 'label1;
        }

        break;
    }
}