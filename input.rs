use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
use std::io::stdin;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
fn main(){
    let mut number:i32=10;
    let mut st=String::new();
    let line=stdin().read_line(&mut st);
    println!("{}", line);
    print_type_of(&line);

    let guess:Option<i32>=line.ok().map_or(None,|_|st.trim().parse().ok());

    match guess 
    {
        Some(n)if n == number => println!("{}", n),
        None => println!("enter a number"),
        _ => println!("rest")
    }


    'outer: loop {
 
        // Define our lucky number
        let number: i32 = 10;
        println!("Pick a Number");
 
        loop {
 
            // Create our string
            let mut line = String::new();
 
            // Pass the reference where we store the string
            // entered on the keyboard
            let input = stdin().read_line(&mut line);
 
            // An Option value is either Some with a value or None
            // ok() means that the reader is at the end of the line
            // map_or() applies a default value, or
            // applies functions to the value
            // trim() removes the newline
            // parse converts the string to a i32
            let guess: Option<i32> = input.ok().map_or(None, |_| line.trim().parse().ok());
 
            match guess {
                None => println!("Enter a Number"),
                Some(n) if n == number => {
                    println!("You Guessed It");
                    break 'outer;
                }
                Some(n) if n < number => println!("Too Low"),
                Some(n) if n > number => println!("Too High"),
                Some(_) => println!("Error")
            }
        }
    }
}