#include <stdio.h>
#include <string.h>

#include "tc.h"
#include "draw.h"

void draw_window(int screen) {

    

    // if (strcmp(screen, "splash") == 0) {
    // if (screen == 1) {
    //     draw_splash();
    // }
    draw_splash();
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
