#include "tools.h"

// Foreground colours

const char TC_NRM[] = "\033[0m";
const char TC_RED[] = "\033[1;31m";
const char TC_GRN[] = "\033[1;32m";
const char TC_YEL[] = "\033[1;33m";
const char TC_BLU[] = "\033[1;34m";
const char TC_MAG[] = "\033[1;35m";
const char TC_CYN[] = "\033[1;36m";
const char TC_WHT[] = "\033[1;37m";

// Background colours

const char TC_BG_NRM[] = "\033[40m";
const char TC_BG_RED[] = "\033[41m";
const char TC_BG_GRN[] = "\033[42m";
const char TC_BG_YEL[] = "\033[43m";
const char TC_BG_BLU[] = "\033[44m";
const char TC_BG_MAG[] = "\033[45m";
const char TC_BG_CYN[] = "\033[46m";
const char TC_BG_WHT[] = "\033[47m";

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
