CPP=g++
CPPFLAGS=-std=c++11 -Wall -Wnon-virtual-dtor -Woverloaded-virtual -Werror -pedantic-errors
CPPTARGET=$(patsubst %.cpp,%, $(wildcard *.cpp)) 

CC=gcc
CFLAGS=-std=c11 -Wall -Werror -pedantic-errors
CTARGET=$(patsubst %.c,%, $(wildcard *.c))

RUSTC=rustc
RUSTSRC=$(wildcard *.rs)
RUSTEXE=$(RUSTSRC:.rs=)
RUSTCOPT=-C opt-level=0 -C debuginfo=2 -C debug-assertions=yes
RUSTFMT=rustfmt
#RUSTFMT=../fmt.sh


all: $(CPPTARGET) $(CTARGET) $(RUSTEXE)

%: %.cpp
	$(CPP) $(CPPFLAGS) -o $@ $<

%: %.c
	$(CC) $(CFLAGS) -o $@ $<

%: %.rs
	$(RUSTFMT) $<
	$(RUSTC) $(RUSTOPT) $<

clean:
	rm -rf *~ a.out $(CPPTARGET) $(CTARGET) $(RUSTEXE) *.dSYM
