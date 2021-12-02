use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
fn main(){
    let vec = vec![1,2,3,4,5,6];
    println!("{:?}",vec.iter());
    let mut it=vec.iter();
    println!("{:?}",it.next());

    println!("{:?}",sum(&vec));
}

fn sum(vec: &Vec<i32>) -> i32 {
    // let sum = vec.iter().fold(0, |mut s,&x| {s+=x;s});
    let sum = vec.iter().fold(0, |mut s,&x| s+x);
    return sum;
}