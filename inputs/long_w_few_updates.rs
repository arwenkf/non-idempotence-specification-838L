#[atomic]
#[nids(x)]
fn update1(x: &mut u16) {
    let mut exec_num = 1;
    'label1: loop {
        *x = 10;
        if exec_num == 1 {
            exec_num += 1;
            continue 'label1;
        }
        break;
    }
}

#[atomic]
#[nids(x)]
fn update2(x: &mut u16) {
    let mut exec_num = 1;
    'label1: loop {
    *x = 20;
    *x = 25;
}

#[atomic]
#[nids(x)]
fn update3(x: &mut u16) {
    let mut exec_num = 1;
    'label1: loop {
    *x = *x + 5;
}

#[atomic]
#[nids(x)]
fn update4(x: &mut u16) {
    let mut exec_num = 1;
    'label1: loop {
    *x = 0;
}

#[atomic]
#[nids(x)]
fn update5(x: &mut u16) {
    let mut exec_num = 1;
    'label1: loop {
    *x = 100;
    *x = 50;
    *x = 100;
}

#[atomic]
#[nids(x)]
fn update6(x: &mut u16) {
    *x = 0xFFFF;
}

#[atomic]
#[nids(x)]
fn update7(x: &mut u16) {
    *x = 123;
}

#[atomic]
#[nids(x)]
fn update8(x: &mut u16) {
    *x = 8;
    *x = 3;
    *x = 8;
}

#[atomic]
#[nids(x)]
fn main() {
    let mut x: u16 = 0;

    update1(&mut x);
    update2(&mut x);
    update3(&mut x);
    update4(&mut x);
    update5(&mut x);
    update6(&mut x);
    update7(&mut x);
    update8(&mut x);
}