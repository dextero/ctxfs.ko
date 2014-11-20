#
# Makefile for the linux ctxext2-filesystem routines.
#

obj-$(CONFIG_EXT2_FS) += ctxext2.o

ctxext2-y := balloc.o dir.o file.o ialloc.o inode.o ioctl.o namei.o super.o symlink.o

ctxext2-$(CONFIG_CTXEXT2_FS_XATTR)     += xattr.o xattr_user.o xattr_trusted.o
ctxext2-$(CONFIG_CTXEXT2_FS_POSIX_ACL) += acl.o
ctxext2-$(CONFIG_CTXEXT2_FS_SECURITY)  += xattr_security.o
ctxext2-$(CONFIG_CTXEXT2_FS_XIP)       += xip.o
