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

#pragma GCC diagnostic ignored "-Wdeclaration-after-statement"

#define CTXFS_MAGIC 0x000C78F5

#define DEBUG 1

#define MAKE_PRINT(func, format, ...) \
    func("ctxfs [%s @ %s:%d]: " format "\n", \
         __func__, __FILE__, __LINE__, ##__VA_ARGS__)

#if DEBUG
#   define trace(...) MAKE_PRINT(pr_info, __VA_ARGS__)
#else
#   define trace(...)
#endif

#define prinfo(...) MAKE_PRINT(pr_info, __VA_ARGS__)
#define prerr(...) MAKE_PRINT(pr_err, __VA_ARGS__)

struct inode *ctxfs_alloc_inode(struct super_block *sb) {
    trace("ctxfs_alloc_inode");
    return NULL;
}
void ctxfs_destroy_inode(struct inode *inode) {
    trace("ctxfs_destroy_inode");
}
void ctxfs_dirty_inode(struct inode *inode,
                       int flags) {
    trace("ctxfs_dirty_inode");
}
int ctxfs_write_inode(struct inode *inode,
                      struct writeback_control *wbc) {
    trace("ctxfs_write_inode");
    return -1;
}
int ctxfs_drop_inode(struct inode *inode) {
    trace("ctxfs_drop_inode");
    return -1;
}
void ctxfs_evict_inode(struct inode *inode) {
    trace("ctxfs_evict_inode");
}
void ctxfs_put_super(struct super_block *sb) {
    trace("ctxfs_put_super");
}
int ctxfs_sync_fs(struct super_block *sb,
                  int wait) {
    trace("ctxfs_sync_fs");
    return -1;
}
int ctxfs_freeze_fs(struct super_block *sb) {
    trace("ctxfs_freeze_fs");
    return -1;
}
int ctxfs_unfreeze_fs(struct super_block *sb) {
    trace("ctxfs_unfreeze_fs");
    return -1;
}
int ctxfs_statfs(struct dentry *dentry,
                 struct kstatfs *stat) {
    trace("ctxfs_statfs");
    return -1;
}
int ctxfs_remount_fs(struct super_block *sb,
                     int *foo,
                     char *bar) {
    trace("ctxfs_remount_fs");
    return -1;
}
void ctxfs_umount_begin(struct super_block *sb) {
    trace("ctxfs_umount_begin");
}
int ctxfs_show_options(struct seq_file *seqf,
                       struct dentry *dentry) {
    trace("ctxfs_show_options");
    return -1;
}
int ctxfs_show_devname(struct seq_file *seqf,
                       struct dentry *dentry) {
    trace("ctxfs_show_devname");
    return -1;
}
int ctxfs_show_path(struct seq_file *seqf,
                    struct dentry *dentry) {
    trace("ctxfs_show_path");
    return -1;
}
int ctxfs_show_stats(struct seq_file *seqf,
                     struct dentry *dentry) {
    trace("ctxfs_show_stats");
    return -1;
}
int ctxfs_bdev_try_to_free_page(struct super_block *sb,
                                struct page *page,
                                gfp_t foo) {
    trace("ctxfs_bdev_try_to_free_page");
    return -1;
}
long ctxfs_nr_cached_objects(struct super_block *sb,
                             int foo) {
    trace("ctxfs_nr_cached_objects");
    return -1;
}
long ctxfs_free_cached_objects(struct super_block *sb,
                               long foo,
                               int bar) {
    trace("ctxfs_free_cached_objects");
    return -1;
}

static const struct super_operations CTXFS_SUPER_OPS = {
    .alloc_inode = ctxfs_alloc_inode,
    .destroy_inode = ctxfs_destroy_inode,

    .dirty_inode = ctxfs_dirty_inode,
    .write_inode = ctxfs_write_inode,
    .drop_inode = ctxfs_drop_inode,
    .evict_inode = ctxfs_evict_inode,
    .put_super = ctxfs_put_super,
    .sync_fs = ctxfs_sync_fs,
    .freeze_fs = ctxfs_freeze_fs,
    .unfreeze_fs = ctxfs_unfreeze_fs,
    .statfs = ctxfs_statfs,
    .remount_fs = ctxfs_remount_fs,
    .umount_begin = ctxfs_umount_begin,

    .show_options = ctxfs_show_options,
    .show_devname = ctxfs_show_devname,
    .show_path = ctxfs_show_path,
    .show_stats = ctxfs_show_stats,
    .bdev_try_to_free_page = ctxfs_bdev_try_to_free_page,
    .nr_cached_objects = ctxfs_nr_cached_objects,
    .free_cached_objects = ctxfs_free_cached_objects
};

static int ctxfs_fill_sb(struct super_block *sb,
                         void *data,
                         int silent) {
    trace("begin");
    struct inode *root = NULL;

    trace("s_type = %p", sb->s_type);
    trace("s_op = %p", sb->s_op);
    trace("dq_op = %p", sb->dq_op);
    trace("s_qcop = %p", sb->s_qcop);
    trace("s_export_op = %p", sb->s_export_op);
    trace("s_root = %p", sb->s_root);
    trace("s_security = %p", sb->s_security);
    trace("s_xattr = %p", sb->s_xattr);
    trace("s_bdev = %p", sb->s_bdev);
    trace("s_bdi = %p", sb->s_bdi);
    trace("s_mtd = %p", sb->s_mtd);
    trace("s_fs_info = %p", sb->s_fs_info);
    trace("s_subtype = %p", sb->s_subtype);
    trace("s_options = %p", sb->s_options);
    trace("s_d_op = %p", sb->s_d_op);
    trace("s_dio_done_wq = %p", sb->s_dio_done_wq);

    sb->s_magic = CTXFS_MAGIC;
    sb->s_op = &CTXFS_SUPER_OPS;

    root = new_inode(sb);
    if (!root) {
        prerr("new_inode failed");
        return -ENOMEM;
    }

    root->i_ino = 0;
    root->i_sb = sb;
    root->i_atime = root->i_mtime = root->i_ctime = CURRENT_TIME;
    inode_init_owner(root, NULL, S_IFDIR);

    sb->s_root = d_make_root(root);
    if (!sb->s_root) {
        prerr("d_make_root failed");
        return -ENOMEM;
    }

    return 0;
}

static struct dentry *ctxfs_mount(struct file_system_type *fs_type,
                                  int flags,
                                  const char *dev,
                                  void *data) {
    trace("begin");
    struct dentry *entry = mount_nodev(fs_type, flags, data, ctxfs_fill_sb);
    trace("entry = %p", entry);

    if (IS_ERR(entry)) {
        prerr("mount failed: %ld", PTR_ERR(entry));
    } else {
        prinfo("mount succeeded");
    }

    return entry;
}

static struct file_system_type FS_TYPE = {
    .name = "ctxfs",
    .fs_flags = 0,
    .mount = ctxfs_mount,
    .kill_sb = kill_block_super,
    .owner = THIS_MODULE
};

static int __init simple_init(void) {
    int result = register_filesystem(&FS_TYPE);
    if (result) {
        prerr("register_filesystem failed (%d)", result);
    } else {
        prinfo("registered");
    }
    return 0;
}

static void __exit simple_exit(void) {
    prinfo("exit");
    if (unregister_filesystem(&FS_TYPE)) {
        prerr("unregister_filesystem failed");
    }
}

module_init(simple_init);
module_exit(simple_exit);
