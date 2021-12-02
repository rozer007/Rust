use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
fn main() {
    // let mut t = 90.0;
    // print_type_of(&t);
    let mut str= "hello world it's zombie time";
    println!("{}", str.len());
    let (f,e)=str.split_at(12);
    println!("{} - {}",f,e);
    let mut spt =str.split(" "); // return ans iterator
    print_type_of(&spt);

    let mut word=spt.next();
    loop {
        match word
        {
            Some(r)=> println!("{}", r),
            None=>break,
        }
        word=spt.next();
    }
    
    println!("{}",str);
    let mut wtsp=str.split_whitespace();
    
    let mut word=wtsp.next();
    loop {
        match word
        {
            Some(r)=> println!("{}", r),
            None=>break,
        }
        word=wtsp.next();
    }
    
    let mut count=str.chars().count();
    println!("{}",count);

    let mut charr=str.chars();  //split the string to character
    print_type_of(&charr);
    let mut word=charr.next();
    loop {
        match word
        {
            Some(r)=> println!("{}", r),
            None=>break,
        }
        word=charr.next();
    }

    //iterate through the lines of the string

    let rand_string2 = "I am a random string\nThere are other strings like it\nThis string is the best";
    let mut lines = rand_string2.lines(); //split the string by the newline
    print_type_of(&lines);
    let mut indiv_line = lines.next();
 
    loop {
        match indiv_line {
            Some(x) => println!("{}", x),
            None => break,
        }
        indiv_line = lines.next();
    }

}