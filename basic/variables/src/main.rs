fn main() {
    mutable_case();
    lesson_end();
    const_case();
    lesson_end();
    var_redefine_case();
    lesson_end();
}


fn lesson_end(){
    println!("----------------")
}
//=============================================
fn mutable_case() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

//=============================================
fn const_case(){
    println! ("MAX_POINTS is: {}", MAX_POINTS);
    const MAX_POINTS: u32 = 100_000;
    print_another_const();
}

fn print_another_const() {
    println!("MAX_POINTS is: {}", MAX_POINTS);
}

const MAX_POINTS: u32 = 200_000;

//=============================================
fn var_redefine_case(){
    let x = "_";
    println!("The value of x is: {}", x);
    let x = x.len();
    println!("The value of x is: {}", x);
    let x = "Привет!";
    println!("The value of x is: {}", x);
    let x: u32 = x.len() as u32;
    println!("The value of x is: {}", x);
    let x = "Привет!";
    println!("The value of x is: {}", x);
    let x: f32 = 3.45;
    println!("The value of x is: {}", x);
}