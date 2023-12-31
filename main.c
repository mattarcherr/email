#include <stdio.h>
#include <signal.h>
#include "tc.h"


void window_resized() {
    printf("%s", "\x1B[41m");
    TC_CLR_SCR();

    int x, y;
    tc_get_cols_rows(&x, &y);

    tc_move_cursor((x-6)/2, y/2);
    printf("%sVIMAIL\n", TC_BLU);
}

int main() {

    // signal(SIGWINCH, &window_resized);
    // tc_echo_off();
    // tc_hide_cursor();
    //
    // printf("%s", "\x1B[41m");
    // TC_CLR_SCR();
    //
    // int x, y;
    // tc_get_cols_rows(&x, &y);
    //
    // tc_move_cursor((x-6)/2, y/2);
    tc_set_colour("#7C7C7C", "#111111", "Hello");

    getchar();
    tc_show_cursor();
    return 0;
}
