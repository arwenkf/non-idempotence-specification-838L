#[atomic]
#[nids(x)]
fn update8(x: &mut u16) {
    *x = 8;
    *x = 3;
    *x = 8;
}

#[atomic]
#[nids(x)]
fn update7(x: &mut u16) {
    *x = 7;
    update8(x);
}

#[atomic]
#[nids(x)]
fn update6(x: &mut u16) {
    *x = 6;
    update7(x);
}

#[atomic]
#[nids(x)]
fn update5(x: &mut u16) {
    *x = 5;
    update6(x);
}

#[atomic]
#[nids(x)]
fn update4(x: &mut u16) {
    *x = 4;
    update5(x);
}

#[atomic]
#[nids(x)]
fn update3(x: &mut u16) {
    *x = 3;
    update4(x);
}

#[atomic]
#[nids(x)]
fn update2(x: &mut u16) {
    *x = 2;
    update3(x);
}

#[atomic]
#[nids(x)]
fn update1(x: &mut u16) {
    *x = 1;
    update2(x);
}

#[atomic]
#[nids(x)]
fn main() {
    let mut x: u16 = 0;
    update1(&mut x);
}