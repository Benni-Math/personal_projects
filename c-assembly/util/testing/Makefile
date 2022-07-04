# General Makefile (modify for use)
P=gsl_erf
OBJECTS=
# need to put glib-2.0 to #include <glib.h>
# gsl is also needed because the base lib has arm64 incompatibility issues
# i.e. I compiled gsl from source into /usr/local/
CFLAGS =`pkg-config --cflags glib-2.0` -Wall -O3 -std=gnu11
LDLIBS=`pkg-config --libs glib-2.0 gsl`
CC=gcc

$(P): $(OBJECTS)
