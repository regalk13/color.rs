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

    pub struct Colored;
    impl Colored {
        pub fn text_to_color(text: &str, color: Color) -> String {
            let print = "\u{001b}[38;5;".to_owned() + &color.code +"m "+ &text;
            print.to_string()
        }

        pub fn text_to_color_8colors(text: &str, color: Color) -> String {
            let print = "\u{001b}[".to_owned() + &color.code +";1m "+ &text;
            print.to_string()
        }
    }

}
