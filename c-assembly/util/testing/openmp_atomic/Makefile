# General Makefile (modify for use)
P=openmp_test
OBJECTS=
# need to put glib-2.0 to #include <glib.h>
# gsl is also needed because the base lib has arm64 incompatibility issues
# i.e. I compiled gsl from source into /usr/local/
CFLAGS =`pkg-config --cflags glib-2.0` -Wall -O3 -Xpreprocessor -fopenmp -pthread -std=gnu11
LDLIBS=-L/opt/homebrew/Cellar/gcc/11.3.0_2/lib/gcc/11 -L/opt/homebrew/lib `pkg-config --libs glib-2.0 gsl` -lomp -latomic
CC=gcc

$(P): $(OBJECTS)
