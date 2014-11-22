KERNEL_VERSION := $(shell uname -r)

EXTRA_CFLAGS = -DCONFIG_EXT2_CTXFS

ctxext2-y := balloc.o dir.o file.o ialloc.o inode.o ioctl.o namei.o super.o symlink.o
ctxext2-y += ext2_ctx.o

ctxext2-$(CONFIG_CTXEXT2_FS_XATTR)     += xattr.o xattr_user.o xattr_trusted.o
ctxext2-$(CONFIG_CTXEXT2_FS_POSIX_ACL) += acl.o
ctxext2-$(CONFIG_CTXEXT2_FS_SECURITY)  += xattr_security.o
ctxext2-$(CONFIG_CTXEXT2_FS_XIP)       += xip.o

obj-m += ctxext2.o
ctxext2-objs := $(ctxext2-y)

.PHONY: all
all:
	make -C /lib/modules/$(KERNEL_VERSION)/build M=$(PWD) modules

.PHONY: clean
clean:
	make -C /lib/modules/$(KERNEL_VERSION)/build M=$(PWD) clean

