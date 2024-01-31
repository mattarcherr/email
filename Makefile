CC=gcc

default: vimail

vimail: main.o tools.o control.o draw.o
	$(CC) main.o tools.o control.o draw.o -o $@

main.o: main.c
	$(CC) -c main.c 

tools.o: tools.c tools.h
	$(CC) -c tools.c

control.o: control.c control.h
	$(CC) -c control.c

draw.o: draw.c draw.h
	$(CC) -c draw.c

clean:
	rm main.o tc.o vimail
