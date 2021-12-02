use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main()
{
    let mut st = String::new();
    stdin().read_line(&mut st).unwrap();
    let num : i32 =st.trim().parse().unwrap();

    let mut vec : Vec<i32>= Vec::new();

    for i in 0..num
    {
        let mut st = String::new();
        stdin().read_line(&mut st).unwrap();
        let num : i32 =st.trim().parse().unwrap();
        vec.push(num);
    }
    let max = *vec.iter().max().unwrap();
    let mut i=0;
    
    while(i < max)
    {
        let mut j=0;
        while(j<vec.len())
        {
            if(vec[j]>=max-i)
            {
                print!("*\t");
            }
            else 
            {
                print!("\t");
            }
            j=j+1;
        }
        i=i+1;
        println!();
    }
}