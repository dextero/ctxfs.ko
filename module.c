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

char __morestack[1024];
extern struct module __this_module;

static struct file_system_type FS_TYPE = {
    .name = "rustfs",
    .fs_flags = 0,
    .owner = THIS_MODULE
};

extern int rustfs_module_init(struct file_system_type *fs_type);
extern void rustfs_module_exit(struct file_system_type *fs_type);

static int __init simple_init(void) {
    int retval;
    printk(KERN_INFO "rustfs: calling rustfs_module_init\n");
    retval = rustfs_module_init(&FS_TYPE);
    printk(KERN_INFO "ristfs: init complete\n");
    return retval;
}

static void __exit simple_exit(void) {
    printk(KERN_INFO "rustfs: exit\n");
    rustfs_module_exit(&FS_TYPE);
}

module_init(simple_init);
module_exit(simple_exit);
