use coloring_terminal::color::{Color, Colored};

fn main() {
    let color_random = Color::new("random", 31);
    println!("{}", String::from("Test").text_to_color_8colors(color_random));
}
