use coloring_terminal::color::{Color, Colored};

fn main() {
    let color_random = Color::new("random", 31);
    println!("{}", Colored::text_to_color_8colors("Test", color_random));
}
