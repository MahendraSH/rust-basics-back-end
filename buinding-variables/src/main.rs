fn main() {
    let mut x: i32;
    // let x :i32 - > gives errror
    //
    x = 1;
    x += 100;
    println!("{x}");
    assert_eq!(x, 101);
    println!("success");
}
