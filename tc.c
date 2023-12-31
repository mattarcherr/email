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

void tc_set_colour(char fg[], char bg[], char text[]){

    int fg_R, fg_G, fg_B = 0;
    int bg_R, bg_G, bg_B = 0;

    fg_R = hex_to_dec_2d((char[]) {fg[1], fg[2]});
    fg_G = hex_to_dec_2d((char[]) {fg[3], fg[4]});
    fg_B = hex_to_dec_2d((char[]) {fg[5], fg[6]});

    bg_R = hex_to_dec_2d((char[]) {bg[1], bg[2]});
    bg_G = hex_to_dec_2d((char[]) {bg[3], bg[4]});
    bg_B = hex_to_dec_2d((char[]) {bg[5], bg[6]});


    char fg_str[50];
    char bg_str[20];
    sprintf(fg_str, "\033[38;2;%i;%i;%i\033[48;2;%i;%i;%im%s", fg_R, fg_G, fg_B, bg_R, bg_G, bg_B, text); // fg
    puts(fg_str); // fg
}

const char * tc_get_colour(char hex[]){

    int R, G, B = 0;

    R = hex_to_dec_2d((char[]) {hex[1],hex[2]});
    G = hex_to_dec_2d((char[]) {hex[3],hex[4]});
    B = hex_to_dec_2d((char[]) {hex[5],hex[6]});



    char* str = malloc(25);
    sprintf(str, "\033[48;2;%i;%i;%im", R, G, B); // bg
    return str;
}

// get size of window
void tc_get_cols_rows(int *cols, int *rows){

	struct winsize size;
	ioctl(1, TIOCGWINSZ, &size);
	*cols = size.ws_col;
	*rows = size.ws_row;
}

// disable console echo attr
void tc_echo_off(){

	struct termios term;
	tcgetattr(1, &term);
	term.c_lflag &= ~ECHO;
	tcsetattr(1, TCSANOW, &term);
}

// enable console echo attr
void tc_echo_on(){

	struct termios term;
	tcgetattr(1, &term);
	term.c_lflag |= ECHO;
	tcsetattr(1, TCSANOW, &term);

}
