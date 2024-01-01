CC=gcc

default: vimail

vimail: main.o tc.o control.o
	$(CC) main.o tc.o control.o -o $@

main.o: main.c
	$(CC) -c main.c 

tc.o: tc.c tc.h
	$(CC) -c tc.c

control.o: control.c control.h
	$(CC) -c control.c

clean:
	rm main.o tc.o vimail
