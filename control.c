#include <stdio.h>

#include "control.h"
#include "draw.h"

void switchKhit(char c) {

    // CONTROLS FOR SPLASH SCREEN
    if (get_program_screen() == SPLASH) {
        switch(c) {
            case 'a':
                printf("AYY");
                break;
            case 'b':
                printf("BEE");
                break;
            case '1':
                set_program_screen(HOME);
                draw_window();
                break;
        }
    }
    // CONTROLS FOR HOME SCREEN
    else if (get_program_screen() == HOME) {
        switch(c) {
            case 'a':
                printf("A");
                break;
            case 'b':
                printf("B");
                break;
        }
    }
}
