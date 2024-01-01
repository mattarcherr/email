#include <stdio.h>
#include <termios.h>
#include <signal.h>
#include "tc.h"


void draw_window() {
    printf("%s", "\033[47m");
    tc_clr_screen();

    int x, y;
    tc_get_cols_rows(&x, &y);

    tc_move_cursor((x-6)/2, (y/2)-3);
    printf("%s%sVIMAIL\n", TC_BG_WHT, "\033[38;5;0m");


    char emailaddr1[] = "matthewarcherr@gmail.com";
    int emaillen= strlen(emailaddr1)+5;
    tc_move_cursor((x-emaillen)/2, (y/2)+2);
    printf("%s%s(1) %s\n", TC_BG_WHT, "\033[38;5;0m", emailaddr1);
}

void setup() {

    tc_alter_termflag(~ECHO);
    tc_alter_termflag(~ICANON);
    tc_enter_alt_screen();
    tc_hide_cursor();
}

int main() {

    signal(SIGWINCH, &draw_window);

    struct termios init_term;
    tcgetattr(1, &init_term);
    setup();

    draw_window();

    char c;

    do {

        c = getchar();
    } while (c != 'q');

    tc_show_cursor();
    tc_exit_alt_screen();
    tcsetattr(1, TCSANOW, &init_term);
    return 0;
}
