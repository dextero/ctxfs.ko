RUST_ROOT := /usr/local

-include ./config.mk

RC := $(RUST_ROOT)/bin/rustc

KERNEL_VERSION := $(shell uname -r)
C_SOURCES := $(shell find ./ -maxdepth 1 -name '*.c' | sed 's|^\.\/||')
C_OBJECTS := $(C_SOURCES:.c=.o)
RUST_SOURCES := $(shell find ./ -maxdepth 1 -name '*.rs' | sed 's|^\.\/||')
RUST_OBJECTS := $(RUST_SOURCES:.rs=.o)
MODULE := rustfs

obj-m = $(MODULE).o
rustfs-objs := module.o main.o

.PHONY: all
all: $(MODULE).ko

$(MODULE).ko: $(C_SOURCES) $(RUST_OBJECTS) fixup
	make -C /lib/modules/$(KERNEL_VERSION)/build M=$(PWD) modules
	./fixup $@

fixup:
	$(RC) fixup.rs

%.o: %.rs
	$(RC) -L $(PWD) -O --crate-type lib -o $@ --emit obj $<

.PHONY: clean
clean:
	make -C /lib/modules/$(KERNEL_VERSION)/build M=$(PWD) clean
	rm -f fixup

