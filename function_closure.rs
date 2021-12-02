use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
fn main() 
{
    hello("zombie");
    println!("2 + 65 = {}",sum(2,65));
    
    //closure
    let sum=|x:i32 ,y:i32|x-y;
    println!("{}",sum(23,23));

}
fn hello(name: &str) {
    println!("Hello { }", name);
}

fn sum(num1: i32, num2: i32)-> i32 {
    num1 + num2
}
