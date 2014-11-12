-include ./config.mk

KERNEL_VERSION := $(shell uname -r)

obj-m = ctxfs.o
ctxfs-objs := module.o

.PHONY: all
all:
	make -C /lib/modules/$(KERNEL_VERSION)/build M=$(PWD) modules

clean:
	make -C /lib/modules/$(KERNEL_VERSION)/build M=$(PWD) clean
