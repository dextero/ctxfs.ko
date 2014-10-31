RUST_ROOT := /usr/local

-include ./config.mk

RC := $(RUST_ROOT)/bin/rustc

KERNEL_VERSION = $(shell uname -r)
C_SOURCES = $(shell find ./ -name '*.c' | sed 's|^\.\/||')
C_OBJECTS = $(C_SOURCES:.c=.o)
RUST_SOURCES = $(shell find ./ -name '*.rs' | sed 's|^\.\/||')
RUST_OBJECTS = $(RUST_SOURCES:.rs=.o)
MODULE = rustfs.ko

obj-m = $(OBJECTS) $(RUST_OBJECTS)
rustfs-objs := $(OBJECTS)

.PHONY: all
all: $(MODULE)

$(MODULE): $(C_SOURCES) $(RUST_OBJECTS) fixup
	make -C /lib/modules/$(KERNEL_VERSION)/build M=$(PWD) modules
	#./fixup $@

fixup:
	# fixup.rs
	#$(RC) fixup.rs

%.o: %.rs
	$(RC) -O --crate-type lib -o $@ --emit obj $<

.PHONY: clean
clean:
	make -C /lib/modules/$(KERNEL_VERSION)/build M=$(PWD) clean
