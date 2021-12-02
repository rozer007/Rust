#[derive(Debug)]
enum Hero {
    Fast,
    Strong(i32),
    Info {name: String, secret: String}
}
 
// Receives enum
fn get_info(h: Hero){
    match h {
        Hero::Fast => println!("Fast"),
        Hero::Strong(i) => println!("Lifts {} tons", i),
        Hero::Info {name, secret} => {
            println!("{} is {}", name, secret);
        },
    }
}

fn main(){
    let hulk = Hero::Strong(100);
    let quicksilver = Hero::Fast;
 
     // to_owned() converts a string literal into
     // a String
    let spiderman = Hero::Info {name: "Spiderman".to_owned(),
        secret: "Peter Parker".to_owned()};
 
    // get_info(hulk);
    get_info(spiderman);

    println!("{:?} ", quicksilver);
    println!("{:?} ", hulk);
}