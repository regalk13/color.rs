fn colored(a: i32, b: i32, text: String) -> String { 
    let code = (a * 16 + b).to_string();
    let print = "\u{001b}[38;5;".to_owned() + &code +"m "+ &text;
    format!("{}", print)
}


fn main() {
    println!("{}", colored(13, 16, "test".to_string()));
    let color = "\u{001b}[31m A".to_string();
    println!("{}", color); 

}
