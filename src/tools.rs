pub fn draw_line_h (y: u16, start: u16, end: u16, colour: console::Style) {
    let term = console::Term::stdout(); 
    for i in start..end {
        term.move_cursor_to((start+(end-i)).into(), y.into()).unwrap();
        println!("{}", colour.apply_to("─")); 
    }
}
pub fn draw_line_v (x: u16, start: u16, end: u16, colour: console::Style) {
    let term = console::Term::stdout();
    for i in start..end {
        term.move_cursor_to(x.into(), (start+(end-i)).into()).unwrap();
        println!("{}", colour.apply_to("│")); 
    }
}
pub fn draw_thick_line_h (y: u16, start: u16, end: u16, colour: console::Style) {
    let term = console::Term::stdout();
    for i in start..end {
        term.move_cursor_to((start+(end-i)).into(), y.into()).unwrap();
        println!("{}", colour.apply_to("━")); 
    }
}
pub fn draw_thick_line_v (x: u16, start: u16, end: u16, colour: console::Style) {
    let term = console::Term::stdout();
    for i in start..end {
        term.move_cursor_to(x.into(), (start+(end-i)).into()).unwrap();
        println!("{}", colour.apply_to("┃"));
    }
}
pub fn draw_box(x: u16, y: u16, width: u16, height: u16, colour: console::Style) {
    let term = console::Term::stdout();
    draw_line_h(y, x, x+width, colour.clone());
    draw_line_h(y+height, x, x+width, colour.clone());
    draw_line_v(x, y, y+height, colour.clone());
    draw_line_v(x+width, y, y+height, colour.clone());

    term.move_cursor_to(x.into(), y.into()).unwrap();
    println!("{}", colour.clone().apply_to("┌"));

    term.move_cursor_to((x+width).into(), y.into()).unwrap();
    println!("{}", colour.clone().apply_to("┐"));

    term.move_cursor_to(x.into(), (y+height).into()).unwrap();
    println!("{}", colour.clone().apply_to("└"));

    term.move_cursor_to((x+width).into(), (y+height).into()).unwrap();
    println!("{}", colour.clone().apply_to("┘"));
}
pub fn draw_thick_box(x: u16, y: u16, width: u16, height: u16, colour: console::Style) {
    let term = console::Term::stdout();
    draw_thick_line_h(y, x, x+width, colour.clone());
    draw_thick_line_h(y+height, x, x+width, colour.clone());
    draw_thick_line_v(x, y, y+height, colour.clone());
    draw_thick_line_v(x+width, y, y+height, colour.clone());

    term.move_cursor_to(x.into(), y.into()).unwrap();
    println!("{}", colour.clone().apply_to("┏"));

    term.move_cursor_to((x+width).into(), y.into()).unwrap();
    println!("{}", colour.clone().apply_to("┓"));

    term.move_cursor_to(x.into(), (y+height).into()).unwrap();
    println!("{}", colour.clone().apply_to("┗"));

    term.move_cursor_to((x+width).into(), (y+height).into()).unwrap();
    println!("{}", colour.clone().apply_to("┛"));
}

// pub fn clear_area(x: u16, y: u16, width: u16, height: u16, colour: &str) {

//     for ypos in 0..height {
//         for xpos in 0..width {
//             println!("{}{} ", termion::cursor::Goto(x+xpos, y+ypos), colour)
//         }
//     }
// }
