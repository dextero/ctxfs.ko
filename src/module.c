#ifndef COMMON_H
#define COMMON_H

#include <linux/init.h>
#include <linux/module.h>
#include <linux/slab.h>
#include <linux/fs.h>
#include <linux/errno.h>
#include <linux/types.h>
#include <linux/proc_fs.h>
#include <linux/fcntl.h>
#include <linux/module.h>
#include <linux/sched.h>
#include <linux/stat.h>
#include <linux/namei.h>
#include <linux/device.h>
#include <linux/cdev.h>
#include <linux/mount.h>

#include <asm/uaccess.h>

MODULE_LICENSE("GPL");

extern void rustfs_module_init(void);
extern void rustfs_module_exit(void);

static int __init simple_init(void) {
    printk(KERN_INFO "rustfs: calling rustfs_module_init\n");
    rustfs_module_init();
    printk(KERN_INFO "ristfs: init complete\n");
}

static void __exit simple_exit(void) {
    printk(KERN_INFO "rustfs: exit\n");
    rustfs_module_exit();
}

module_init(simple_init);
module_exit(simple_exit);


#endif // COMMON_H
