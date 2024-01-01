#include "tc.h"


int hex_to_dec_2d(char hex[]) {

    int value = 0;

    if (isdigit(hex[1])) {
        value += ( hex[1] - '0' );
    }
    else { switch(hex[1]) {
        case 'A':
            value += 10; break;
        case 'B':
            value += 11; break;
        case 'C':
            value += 12; break;
        case 'D':
            value += 13; break;
        case 'E':
            value += 14; break;
        case 'F':
            value += 15; break;
    }}
    if (isdigit(hex[0])) {
        value += (hex[0] - '0') * 16;
    }
    else { switch(hex[0]) {
        case 'A':
            value += 160; break;
        case 'B':
            value += 176; break;
        case 'C':
            value += 192; break;
        case 'D':
            value += 208; break;
        case 'E':
            value += 224; break;
        case 'F':
            value += 240; break;
    }}
    return value;
}

// get size of window
void tc_get_cols_rows(int *cols, int *rows){

	struct winsize size;
	ioctl(1, TIOCGWINSZ, &size);
	*cols = size.ws_col;
	*rows = size.ws_row;
}

void tc_alter_termflag(const tcflag_t flag) {

	struct termios term;
	tcgetattr(1, &term);
	term.c_lflag &= ( flag );
	tcsetattr(1, TCSANOW, &term);
}
