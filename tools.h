#ifndef TC_H
#define TC_H

#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>
#include <string.h>
#include <sys/ioctl.h>
#include <termios.h>
// #include <unistd.h>


// TC for terminal control
#define TC_NRM  "\033[0m"
#define TC_RED  "\033[1;31m"
#define TC_GRN  "\033[1;32m"
#define TC_YEL  "\033[1;33m"
#define TC_BLU  "\033[1;34m"
#define TC_MAG  "\033[1;35m"
#define TC_CYN  "\033[1;36m"
#define TC_WHT  "\033[1;37m"

// Background colours
#define TC_BG_NRM "\033[40m"
#define TC_BG_RED "\033[41m"
#define TC_BG_GRN "\033[42m"
#define TC_BG_YEL "\033[43m"
#define TC_BG_BLU "\033[44m"
#define TC_BG_MAG "\033[45m"
#define TC_BG_CYN "\033[46m"
#define TC_BG_WHT "\033[47m"

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
