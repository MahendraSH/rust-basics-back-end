fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 10;
        println!(" value of x = {} , value of y = {}", x, y);
    }
    // println!(" value of x = {} , value of y = {}", x, y); -> //  can`t find y in this scope
    function_scoping();
    println!();
    shadowing_1();
    println!();
    shadowing_2();
    println!();
    shadowing_3();
    println!();
}
fn function_scoping() {
    println!("function_scoping");
    let z: i32 = 100;
    // println!(" value of x = {} , value of z = {}", x, z); -> // can`t find the x in this scope
    println!(" value of z = {z}");
}

fn shadowing_1() {
    println!("shadowing_1");
    let x: i32 = 100; // give warring that this is un_unsed
    let x = " some text ";
    println!(" The value of x = {x}"); //the out put - > The value of x = some text
}
fn shadowing_2() {
    println!("shadowing_2");
    let x: i32 = 390;
    {
        let x = 434;
        println!(" the value of  x in block = {x}"); //434
    }

    println!("the value of x out side the blcok = {x} "); // 390
    let x = 34434;
    println!(" the value of x re intilizes x = 34434 = -> {x}"); // 34434
}
fn shadowing_3() {
    println!("shadowing_3");
    let mut x = 439;
    x = 428;
    let x = 448;

    ///  x = x+43;
    //  - > this thing gives error  it takes x as im-mutable variable
    println!(" value of x {x}");
}
