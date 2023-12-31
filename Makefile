CC=gcc

default: vimail

vimail: main.o tc.o
	$(CC) main.o tc.o -o $@

main.o: main.c
	$(CC) -c main.c 

tc.o: tc.c tc.h
	$(CC) -c tc.c

clean:
	rm main.o tc.o vimail
