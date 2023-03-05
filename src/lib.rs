#[allow(dead_code)]
pub mod color {
    pub struct Color {
        name: String,
        code: String,
    }

    impl Color {
        pub fn new(name: &str, code: i32) -> Self {
            return Self {
                name: name.to_string(),
                code: code.to_string(),
            }
        }
    }

    pub trait Colored { 
        fn text_to_color(self, color: Color) -> String;
        fn text_to_color_8colors(self, color: Color) -> String; 
    }

    impl Colored for String {
        fn text_to_color(self, color: Color) -> String {
            let print = "\u{001b}[38;5;".to_owned() + &color.code +"m "+ &self;
            print.to_string()
        }

        fn text_to_color_8colors(self, color: Color) -> String {
            let print = "\u{001b}[".to_owned() + &color.code +";1m "+ &self;
            print.to_string()
        }
    }
}
