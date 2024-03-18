use console::Term;

fn main() {
    let term = Term::stdout();
    term.set_title("Vimail");
    println!("[107m[2J");

    let (y, x) = term.size();

    term.move_cursor_to(((x/2)-4).into(), (y/2).into()).unwrap();
    println!("[30mTEST");

    loop {
        if term.read_char().unwrap() == 'q' {
            println!("[0m    [2J");
            break;
        }
    }

}
