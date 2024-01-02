#include <stdio.h>
#include <termios.h>
#include <signal.h>

#include "tc.h"
#include "control.h"
// #include "draw.h"


// void draw_window() {
//     printf("%s", "\033[47m");
//     tc_clr_screen();
//
//     int x, y;
//     tc_get_cols_rows(&x, &y);
//
//     tc_move_cursor((x-6)/2, (y/2)-3);
//     printf("%s%sVIMAIL\n", TC_BG_WHT, "\033[38;5;0m");
//
//
//     char emailaddr1[] = "matthewarcherr@gmail.com";
//     int emaillen= strlen(emailaddr1)+5;
//     tc_move_cursor((x-emaillen)/2, (y/2)+2);
//     printf("%s%s(1) %s\n", TC_BG_WHT, "\033[38;5;0m", emailaddr1);
// }

struct termios setup() {

    tc_alter_termflag(~ECHO);
    tc_alter_termflag(~ICANON);
    tc_enter_alt_screen();
    tc_hide_cursor();

    struct termios init_term;
    tcgetattr(1, &init_term);
    return init_term;
}

void setdown(struct termios termAttr) {

    tc_alter_termflag(ECHO);
    tc_alter_termflag(ICANON);
    tc_exit_alt_screen();
    tc_show_cursor();

    tcsetattr(1, TCSANOW, &termAttr);
}


int main() {

    tc_enter_alt_screen();
    // redraw on window size change
    char* test = "splash";
    signal(SIGWINCH, draw_window());

    // set up
    struct termios init_term;
    init_term = setup();

    draw_window();

    // quit on 'q'
    char c;
    do {

        c = getchar();
        switchKhit(c);
    } while (c != 'q');

    // set down
    setdown(init_term);
    tc_exit_alt_screen();
    return 0;
}
