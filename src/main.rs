#[derive(Clone)]
struct Color {
    name: String,
    code: String,
}


impl Color {
    fn new(name: &str, a: i32, b: i32) -> Self {
        return Self {
            name: name.to_string(),
            code: (a * 16 + b).to_string(),
        }
    }
}


struct Colored;

impl Colored {
    fn text_to_color(text: &str, color: Color) -> String {
        let print = "\u{001b}[38;5;".to_owned() + &color.code +"m "+ &text;
        return format!("{}", print);
    }
}

fn main() {
    let color_red = Color::new("red", 13, 16);
    println!("{}, Using the color: {} ", Colored::text_to_color("Regius", color_red.clone()), color_red.name); 
}
