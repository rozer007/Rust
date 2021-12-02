use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
fn main() 
{
    let mut x=0;
    loop{
        if(x%2 == 0)
        {
            println!("Even : {}",x);
            x=x+1;
            continue;
        }
        else if(x>10){break;}

        x=x+1;
    }

    while(x>=0)
    {
        println!("{}",x);
        x=x-1;
    }

    for i in 1..10{
        println!("loololop");
    }

    //loop which return a value
    let mut counter=0;
    let mut val=loop{
        if(x%2 == 0)
        {
            counter+=1;
            x=x+1;
            continue;
        }
        else if(x>10)
        {
            break counter*10;
        }
        x=x+1;
    };
    println!("{}",val);
}