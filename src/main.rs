use coloring_terminal::color::{Color, Colored};

fn main() {
    let color_random = Color::new("random", 24);
    println!("{}", Colored::text_to_color("Test", color_random));
}
