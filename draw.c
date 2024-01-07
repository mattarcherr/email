#include <stdio.h>
#include <string.h>

#include "tc.h"
#include "draw.h"

enum program_screen program_screen = SPLASH;
void set_program_screen(enum program_screen screen) { program_screen = screen; }
enum program_screen get_program_screen() { return program_screen; }

void draw_window() {

    switch(program_screen) {
        case 0:
            // SPLASH
            draw_splash();
            break;
        case 1:
            // HOME
            draw_home();
            break;
    }
}


void draw_splash() {
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

void draw_home() {

    printf("%s", "\033[47m");
    tc_clr_screen();

    int x, y;
    tc_get_cols_rows(&x, &y);

    tc_move_cursor((x-6)/2, (y/2)-3);
    printf("%s%sHOMETEST\n", TC_BG_WHT, "\033[38;5;0m");


    char emailaddr1[] = "matthewarcherr@gmail.com";
    int emaillen= strlen(emailaddr1)+5;
    tc_move_cursor((x-emaillen)/2, (y/2)+2);
    printf("%s%s(1) %s\n", TC_BG_WHT, "\033[38;5;0m", emailaddr1);
}
