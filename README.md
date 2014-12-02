ctxext2.ko - context-aware filesystem
=====================================

Inserting this kernel module allows the 'ctxext2' filesystem type partitions to be mounted under Linux systems.

This project is a modification of the ext2 filesystem kernel module, which detects SSID of the connected WiFi network at mount time and hides all data saved when using a different network. Switching networks after the filesystem is mounted does not affect its visible content, though unmounting and remounting again may do.

The filesystem is fully compatible with ext2 and may be mounted as such to display its entire content. Actually, ctxext2 _is_ an ext2 filesystem - it only uses one of the top-level directories (the one with the same name as the WiFi SSID) as a root folder.

Compiling
---------

Make sure Linux kernel headers are available and run `make` from the root repository directory. If all goes well, you'll get a `ctxext2.ko` file beside the Makefile.

Tested under 64-bit Ubuntu 14.04 and kernel 3.13.0.

Usage
-----

Prepare a filesystem using stock `mkfs.ext2` command and mount is as 'ctxext2' filesystem, for example:

```
$ dd if=/dev/zero of=test.fs bs=1024 count=128000
$ mkfs.ext2 ./test.fs
$ sudo mount -o loop -t ctxext2 ./test.fs /mnt
```

Note: mount will fail if ctxext2 detects no connected WiFi networks.

