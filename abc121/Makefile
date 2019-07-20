HASKELLC=ghc
HASKELLSRC=$(wildcard *.hs)
HASKELLHI=$(HASKELLSRC:.hs=.hi)
HASKELLEXE=$(HASKELLSRC:.hs=)
OBJS=$(wildcard *.o)

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


all: $(CPPTARGET) $(CTARGET) $(RUSTEXE) # $(HASKELLEXE)

%: %.cpp
	$(CPP) $(CPPFLAGS) -o $@ $<

%: %.c
	$(CC) $(CFLAGS) -o $@ $<

#%: %.hs
#	$(HASKELLC) -o $@ $<

%: %.rs
	#$(RUSTFMT) $<
	$(RUSTC) $(RUSTOPT) $<

clean:
	rm -rf $(OBJS) *~ a.out $(HASKELLEXE) $(HASKELLHI) $(CPPTARGET) $(CTARGET) $(RUSTEXE) *.dSYM
