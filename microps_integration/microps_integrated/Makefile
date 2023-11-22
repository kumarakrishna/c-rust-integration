APPS = app/udpc.exe \
	app/udps.exe \
	app/tcpc.exe \
	app/tcps.exe \
	# app/icmp.exe \

TESTS = test/test.exe \

DRIVERS = driver/null.o \
	   driver/loopback.o \

OBJS = util.o \
	net.o \
	ether.o \
	arp.o \
	ip.o \
	icmp.o \
	udp.o \
	tcp.o \
	sock.o \

CFLAGS := $(CFLAGS) -g -W -Wall -Wno-unused-parameter -iquote .


ifeq ($(shell uname),Linux)
	CFLAGS := $(CFLAGS) -pthread -iquote platform/linux
	DRIVERS := $(DRIVERS) platform/linux/driver/ether_tap.o platform/linux/driver/ether_pcap.o
	LDFLAGS := $(LDFLAGS) -lrt
	OBJS := $(OBJS) platform/linux/sched.o platform/linux/intr.o
	RUST_LIB = ../utils_rust/target/release/libutillib.so
endif

ifeq ($(shell uname),Darwin)
	CFLAGS := $(CFLAGS)
	DRIVERS := $(DRIVERS)
	RUST_LIB = ../utils_rust/target/release/libutillib.dylib
	
	$(error This Makefile cannot be run on a Mac. Run in Linux)
endif

.SUFFIXES:
.SUFFIXES: .c .o

.PHONY: all clean

all: $(APPS) $(TESTS)

icmp.exe: icmp.o $(OBJS) $(DRIVERS) $(RUST_LIB)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

icmp.o: icmp.c
	$(CC) $(CFLAGS) -c $< -o $@

$(RUST_LIB):
	echo "Building Rust library"

$(APPS): %.exe : %.o $(OBJS) $(DRIVERS)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

$(TESTS): %.exe : %.o $(OBJS) $(DRIVERS) test/test.h
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

.c.o:
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -rf $(APPS) $(APPS:.exe=.o) $(OBJS) $(DRIVERS) $(TESTS) $(TESTS:.exe=.o) app/icmp.o app/icmp.exe
