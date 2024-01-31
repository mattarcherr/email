#ifndef TOOLS_H
#define TOOLS_H

#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>
#include <sys/ioctl.h>
#include <termios.h>
// #include <unistd.h>


// Foreground colours

extern const char TC_NRM[];
extern const char TC_RED[];
extern const char TC_GRN[];
extern const char TC_YEL[];
extern const char TC_BLU[];
extern const char TC_MAG[];
extern const char TC_CYN[];
extern const char TC_WHT[];

// Background colours

extern const char TC_BG_NRM[];
extern const char TC_BG_RED[];
extern const char TC_BG_GRN[];
extern const char TC_BG_YEL[];
extern const char TC_BG_BLU[];
extern const char TC_BG_MAG[];
extern const char TC_BG_CYN[];
extern const char TC_BG_WHT[];

// Clear Screen
#define tc_clr_screen() puts("\033[2J")

// Move cursor
#define tc_move_cursor(X, Y) printf("\033[%d;%dH", Y, X)

// Toggle alternate screen buffer
#define tc_enter_alt_screen() puts("\033[?1049h\033[H")
#define tc_exit_alt_screen() puts("\033[?1049l")

// Toggle cursor
#define tc_hide_cursor() puts("\033[?25l");
#define tc_show_cursor() puts("\033[?25h");


void tc_alter_termflag(const tcflag_t);

void tc_get_cols_rows(int *cols, int *rows);

#endif
