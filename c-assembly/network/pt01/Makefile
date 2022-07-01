# General Makefile (modify for use)
P=
OBJECTS=
# need to put glib-2.0 to #include <glib.h>
CFLAGS =`pkg-config --cflags glib-2.0` -iquote..  -Wall -O3 -std=gnu11
# gsl is also needed because the base lib has arm64 incompatibility issues
# i.e. I compiled gsl from source into /usr/local/
LDLIBS=`pkg-config --libs glib-2.0 gsl`
CC=gcc

$(P): $(OBJECTS)
