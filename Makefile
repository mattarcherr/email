CC=gcc

default: vimail

vimail: main.o tc.o control.o draw.o
	$(CC) main.o tc.o control.o draw.o -o $@

main.o: main.c
	$(CC) -c main.c 

tc.o: tc.c tc.h
	$(CC) -c tc.c

control.o: control.c control.h
	$(CC) -c control.c

draw.o: draw.c draw.h
	$(CC) -c draw.c

clean:
	rm main.o tc.o vimail
