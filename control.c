#include <stdio.h>

#include "control.h"
#include "draw.h"

void switchKhit(char c) {

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
