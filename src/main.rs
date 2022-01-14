mod _20;

fn main() {
    
    println!("{}", _20::is_valid(String::from("()")));
    println!("{}", _20::is_valid(String::from("()[]{}")));
    println!("{}", _20::is_valid(String::from("(]")));
}
