use std:: {i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};
struct rec{
    h:f64,
    l:f64,
}

fn area(r:&rec)-> f64{
    r.h * r.l
}

// using implementation

impl  rec{
    pub fn recarea(&self)-> f64 
    {
        self.h *self.l
    }
}


//trait

struct circle{
    radius:f64,
}

trait hasarea{
     fn area(&self)->f64;
}

impl hasarea for circle{
    fn area(&self)->f64
    {
        3.19*(self.radius*self.radius)
    }
}

impl hasarea for rec{
     fn area(&self)->f64
    {
        self.l * self.h
    }
}
fn main() {
    let rect = rec{
        h:10.9,
        l:89.7,
    };
    let cir=circle{
        radius:99.09,
    };
    println!("{}", area(&rect));
    println!("{}",rect.recarea());
    println!("{}",rect.area());
    println!("{}",cir.area());
}