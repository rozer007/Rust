use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;
fn main() {
    println!("{0} {1} {0}",10,20);
    println!("{var:>ws$}",var=90,ws=7); //total space =ws   >before
    println!("{var:<ws$}{bt}",var=90,ws=7,bt=90); //total space =ws  <after

    println!("m:{:b} n:{:x} l:{:o}",12,12,12); //:b -binary , :x - hexa ,:o - octal
    println!("{:.2}",90.898989898);  //.2 - 2 digit after point
}