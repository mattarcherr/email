#include "tc.h"

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
