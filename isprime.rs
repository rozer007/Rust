use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

fn main(){
    let mut st = String::new();
    stdin().read_line(&mut st).unwrap();
    let num : i32 =st.trim().parse::<i32>().unwrap();
    let mut flags =true;
    let mut div =2;
    loop {
        if(div*div>num)
        {
            break;
        }
        if(num%div == 0){
            flags = false;
            break;
        }
        div=div+1;

    }
    if(flags==false){
        println!("Not prime");
    }
    else
    {
        println!("Prime");
    }
    
}