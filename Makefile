CC=gcc

default: vimail

vimail: main.o
	$(CC) main.c -o $@

main.o: main.c tc.h
	$(CC) -c main.c 

clean:
	rm main.o vimail
