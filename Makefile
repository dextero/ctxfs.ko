-include ./config.mk

KERNEL_VERSION := $(shell uname -r)

.PHONY: all
all:
	rm -f ctxext2.ko
	make -C /lib/modules/$(KERNEL_VERSION)/build M=$(PWD)/ext2 modules
	cp -f ext2/ctxext2.ko ./

.PHONY: clean
clean:
	rm -f ctxext2.ko
	make -C /lib/modules/$(KERNEL_VERSION)/build M=$(PWD)/ext2 clean

#obj-m = ctxfs.o
#ctxfs-objs := module.o
#
#.PHONY: all
#all:
#	make -C /lib/modules/$(KERNEL_VERSION)/build M=$(PWD) modules
#
#clean:
#	make -C /lib/modules/$(KERNEL_VERSION)/build M=$(PWD) clean
