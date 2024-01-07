#include <stdio.h>
#include <termios.h>
#include <signal.h>

#include "tc.h"
#include "control.h"
#include "draw.h"

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

    // redraw on window size change
    signal(SIGWINCH, draw_window);

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
    return 0;
}
