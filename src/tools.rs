pub fn draw_line_h (y: u16, start: u16, end: u16) { 
    for i in start..end {
        println!("{}{}", termion::cursor::Goto(start+(end-i), y), "─" ); 
    }
}
pub fn draw_line_v (x: u16, start: u16, end: u16) {
    for i in start..end {
        println!("{}{}", termion::cursor::Goto(x, start+(end-i)), "│"); 
    }
}
pub fn draw_thick_line_h (y: u16, start: u16, end: u16) { 
    for i in start..end {
        println!("{}{}", termion::cursor::Goto(start+(end-i), y), "━"); 
    }
}
pub fn draw_thick_line_v (x: u16, start: u16, end: u16) {
    for i in start..end {
        println!("{}{}", termion::cursor::Goto(x, start+(end-i)), "┃"); 
    }
}
pub fn draw_box (x: u16, y: u16, width: u16, height: u16) {
    draw_line_h(y, x, x+width);
    draw_line_h(y+height, x, x+width);
    draw_line_v(x, y, y+height);
    draw_line_v(x+width, y, y+height);

    println!("{}{}", termion::cursor::Goto(x, y), "┌");

    println!("{}{}", termion::cursor::Goto(x+width, y), "┐");

    println!("{}{}", termion::cursor::Goto(x, y+height), "└");

    println!("{}{}", termion::cursor::Goto(x+width, y+height), "┘");
}
pub fn draw_thick_box (x: u16, y: u16, width: u16, height: u16) {
    draw_thick_line_h(y, x, x+width);
    draw_thick_line_h(y+height, x, x+width);
    draw_thick_line_v(x, y, y+height);
    draw_thick_line_v(x+width, y, y+height);

    println!("{}{}", termion::cursor::Goto(x, y), "┏");

    println!("{}{}", termion::cursor::Goto(x+width, y), "┓");

    println!("{}{}", termion::cursor::Goto(x, y+height), "┗");

    println!("{}{}", termion::cursor::Goto(x+width, y+height), "┛");
}

pub fn clear_area(x: u16, y: u16, width: u16, height: u16, colour: &str) {

    for ypos in 0..height {
        for xpos in 0..width {
            println!("{}{} ", termion::cursor::Goto(x+xpos, y+ypos), colour)
        }
    }
}
