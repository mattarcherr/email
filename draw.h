#ifndef DRAW_H
#define DRAW_H

void draw_window();
void draw_splash();
void draw_home();


enum program_screen {
    SPLASH,
    HOME
};
void set_program_screen(enum program_screen screen);

#endif
