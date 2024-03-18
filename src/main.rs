use std::io::{stdin,stdout,Write};
use console::Term;

fn main() {
    let term = Term::stdout();
    term.set_title("Vimail");
    println!("[35mMagenta[0m");
    println!("[107m    [2J");
    

    
    
    

    loop {
        match term.read_char() {
            Ok(v) => {
                if v == 'q' {
                    println!("[0m    [2J");
                    break;
                }
            },
            Err(_) => {}
        }
    }

}
