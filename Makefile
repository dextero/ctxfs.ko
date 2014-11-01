RUST_ROOT := /usr/local

-include ./config.mk

RC := $(RUST_ROOT)/bin/rustc

KERNEL_VERSION := $(shell uname -r)
C_SOURCES := $(shell find ./ -maxdepth 1 -name '*.c' | sed 's|^\.\/||')
C_OBJECTS := $(C_SOURCES:%.c=$(PWD)/%.o)
RUST_SOURCES := $(shell find ./ -maxdepth 1 -name '*.rs' | sed 's|^\.\/||')
RUST_OBJECTS := $(RUST_SOURCES:%.rs=$(PWD)/%.o)
MODULE := rustfs

obj-m = $(MODULE).o
rustfs-objs := module.o main.o

.PHONY: all
all: $(MODULE).ko

$(MODULE).ko: $(C_SOURCES) $(RUST_OBJECTS) fixup
	make -C /lib/modules/$(KERNEL_VERSION)/build M=$(PWD) modules
	$(PWD)/fixup $@

fixup:
	$(RC) fixup.rs

%.rs_E: %.rs
	cat $< | \
		awk '/^#/ { \
				 if ($$0 ~ /^#define|#ifdef|#endif/) { \
					 print \
				 } else { \
					print "_" $$0 \
				 } \
			 } \
			 /^[^#]/ { print }' $< | \
	$(CC) -E -o $@ -x c -
	sed -i -e '/^#/d' -e 's|^_#|#|' $@

%.o: %.rs_E
	$(RC) -L $(PWD) -O --crate-type lib -o $@ --emit obj $<
	rm -f $<

.PHONY: clean
clean:
	make -C /lib/modules/$(KERNEL_VERSION)/build M=$(PWD) clean
	rm -f fixup *.rs_E *.rs_E.tmp

