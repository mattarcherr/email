CC=gcc

default: vimail

vimail: main.o
	$(CC) main.c -o vimail

main.o: main.c
	$(CC) -c main.c -o main.o

clean:
	rm main.o vimail
