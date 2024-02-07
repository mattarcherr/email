#include <stdio.h>
#include <string.h>

#include "tools.h"
#include "draw.h"

enum program_screen program_screen = SPLASH;
void set_program_screen(enum program_screen screen) { program_screen = screen; }
enum program_screen get_program_screen() { return program_screen; }

void draw_window()
{

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


void draw_splash() 
{
    tc_colour_screen(TC_BG_WHT);

    int x, y;
    tc_get_cols_rows(&x, &y);

    tc_move_cursor((x-6)/2, (y/2)-3);
    printf("%sVIMAIL\n", "\033[38;5;0m");

    const char* accounts_addrs[2];
    accounts_addrs[0] = "matthewarcherr@gmail.com";
    accounts_addrs[1] = "john@doe.com";

    for (int i = 0; i < 2; i++) {

        int email_len = strlen(accounts_addrs[i])+5;
        tc_move_cursor((x-email_len)/2, (y/2)+(2+i));
        printf("%s(%i) %s\n", "\033[38;5;0m", i+1, accounts_addrs[i]);
    }
}

void draw_home()
{

    printf("%s", "\033[47m");
    tc_clr_screen();

    int x, y;
    tc_get_cols_rows(&x, &y);

    tc_move_cursor(3, 2);
    printf("%s", TC_BG_RED);

}
