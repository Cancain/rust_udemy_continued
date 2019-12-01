enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8), //tuple
    CmykColor{cyan: u8, magneta: u8, yellow: u8, black: u8} //struct
}

pub fn enums() {   
    let c:Color = Color::CmykColor{cyan: 0, magneta: 0, yellow: 0, black: 255};

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0,0,0) 
            | Color::CmykColor{cyan: _, magneta: _, yellow: _, black: 255} => println!("black"),
        Color::RgbColor(r,g,b) => println!("{},{},{}",r,g,b),
        _ => ()
    }
}
