use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;
fn main() {
    print!("Hello world");
    println!(" Bye");

    let a=10;
    let mut b=12;
    // a=90; will give an error
    // println!("{0} - {1}",a,b);
    // println!("Max i8 {}", i8::MAX);
    // println!("Min i8 {}", i8::MIN);
    // println!("Max i16 {}", i16::MAX);
    // println!("Min i16 {}", i16::MIN);
    // println!("Max i32 {}", i32::MAX);
    // println!("Min i32 {}", i32::MIN);
    // println!("Max i64 {}", i64::MAX);
    // println!("Min i64 {}", i64::MIN);
    // println!("Max isize {}", isize::MAX);
    // println!("Min isize {}", isize::MIN);
    // println!("Max usize {}", usize::MAX);
    // println!("Min usize {}", usize::MIN);
    // println!("Max f32 {}", f32::MAX);
    // println!("Min f32 {}", f32::MIN);
    // println!("Max f64 {}", f64::MAX);
    // println!("Min f64 {}", f64::MIN);

    let y:bool = false;
    let x:char = 'k';
    let (n,m):(i32,i64) =(1,1231);
    println!("{}",m);

    let v=10;
    let v="abc";  // ok because we are creating a new variable
    println!("{}",v);

    let mut y=20;
    // y="bbc"; // will give error because we cannot mutate a variable type
    let mut y="nnm";
    println!("{}",y);
}