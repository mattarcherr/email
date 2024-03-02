
pub fn draw_line_h (x: u16, start: u16, end: u16) { 
    for i in start..end {
        println!("{}{}", termion::cursor::Goto(start+i, x), "━"); 
    }
}
pub fn draw_line_v (y: u16, start: u16, end: u16) {
    for i in start..end {
        println!("{}{}", termion::cursor::Goto(y, start+i), "┃"); 
    }
}
