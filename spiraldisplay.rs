use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main()
{

    let mut st = String::new();
    stdin().read_line(&mut st).expect("failed to read");
    let  n :i32 = st.trim().parse::<i32>().expect("Index entered was not a number");
    // print_type_of(&n);
    // const n1 :u32 = const n;
    
    let mut st = String::new();
    stdin().read_line(&mut st).expect("failed to read");
    let  m :i32 = st.trim().parse::<i32>().expect("Index entered was not a number");
    // print_type_of(&m);
    // const m2 : u32 = const m;



    // let mut arr :[[i32;m as usize];n as usize];

    // let arr = [[i32;m];n];   

    let mut arr :Vec<Vec<i32>>= vec![vec![0; m as usize]; n as usize];

    for i in 0..arr.len() 
    {
        for j in 0..arr[0].len()
        {
            let mut st = String::new();
            stdin().read_line(&mut st).expect("failed to read");
            let num : i32 =st.trim().parse().expect("Index entered was not a number");
            arr[i][j] = num;
        }
    }

    let mut minc=0;
    let mut minr=0;
    let mut maxr=arr.len()-1;
    let mut maxc = arr[0].len()-1;
    println!("{}", arr.len());
    println!("{}", arr[0].len());
    println!("maxrow : {}",maxr);
    println!("maxcolumn: {}",maxc);
    
    let mut tno=n*m;
    // tno=tno+1;
    println!("total: {}",tno);
    let mut count =0;

    println!("OUTPUT");
    
    while(count<tno)
    {
        //left wall
        let mut a=minr;
        while(count < tno && a<=maxr)
        {
            println!("{}",arr[a][minc]);
            println!("c1 : {}",count);
            count=count+1;
            a=a+1;
        }
        minc=minc+1;

        //bottom wall
        let mut b=minc;
        while(count < tno && b<=maxc)
        {
            println!("{}",arr[maxr][b]);
            println!("c2 : {}",count);
            count=count+1;
            b=b+1;
        }
        maxr=maxr-1;

        //right wall
        let mut c=maxr;
        while(count < tno && c>=minr)
        {
            println!("{}",arr[c][maxc]);
            println!("c3 : {}",count);
            count=count+1;
            c=c-1;
        }
        maxc=maxc-1;

        //top wall
        let mut d=maxc;
        while(count < tno && d>=minc)
        {
            println!("{}",arr[minr][d]);
            println!("c4 : {}",count);            
            count=count+1;
            d=d-1;
        }
        minr=minr+1;
    }
}
    // for i in 0..arr.len() 
    // {
    //     for j in 0..arr[0].len()
    //     {
    //         print!("{}",arr[i][j]);
    //     }
    //     println!();
    // }
