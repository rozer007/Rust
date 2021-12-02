use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
fn main() {
    //arrays
    let mut arr =[1,2,3,3,4];
    println!("{:?}",arr);
    println!("{}",arr[2]);
    println!("{}",arr.len());

    let arr1=&arr[0..3];
    println!("{:?}",arr1);
    println!("{:?}",&arr[2..4]);

    //vectors

    let mut vsc=vec![1,2,3,4,5,6,7,8,9];
    println!("{:?}",vsc);
    for i in &vsc{
        println!("{}",i);
    }
    
    vsc.push(11111111);
    println!("{:?}",vsc);
    vsc.pop();
    println!("{:?}",vsc);

    //tuple
    let tu=("popopopop",1,'l',2,3,);
    println!("{:?}",tu);
    println!("{}",tu.4);

}