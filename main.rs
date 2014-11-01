#![allow(ctypes)]
#![no_std]
#![feature(intrinsics)]
#![feature(lang_items)]
#![feature(globs)]

#[lang="sized"]
trait Sized {}

mod rustfs {

extern "rust-intrinsic" {
    pub fn transmute<T, U>(val: T) -> U;
}

const SB_UNFROZEN        : uint = 0;
const SB_FREEZE_WRITE    : uint = 1;
const SB_FREEZE_PAGEFAULT: uint = 2;
const SB_FREEZE_FS       : uint = 3;
const SB_FREEZE_COMPLETE : uint = 4;
const SB_FREEZE_LEVELS   : uint = SB_FREEZE_COMPLETE - 1;

const FS_REQUIRES_DEV      : uint = 1;
const FS_BINARY_MOUNTDATA  : uint = 2;
const FS_HAS_SUBTYPE       : uint = 4;
const FS_USERNS_MOUNT      : uint = 8;	   /* Can be mounted by userns root */
const FS_USERNS_DEV_MOUNT  : uint = 16;    /* A userns mount does not imply MNT_NODEV */
const FS_RENAME_DOES_D_MOVE: uint = 32768; /* FS will handle d_move() during rename() internally. */

const ENOMEM: int = 12;

pub struct dentry;
pub struct super_block;
pub struct module;
pub struct hlist_head;
pub struct lock_class_key;

pub struct file_system_type {
    name: *const u8,
    fs_flags: int,

    mount: fn(fs: *mut file_system_type,
              flags: int,
              path: *const u8,
              data: *mut u8) -> *mut dentry,
    kill_sb: fn(sb: *mut super_block),

    owner: *mut module,
    next: *mut file_system_type,
    fs_supers: *mut hlist_head,

    s_lock_key: lock_class_key,
    s_umount_key: lock_class_key,
    s_vfs_rename_key: lock_class_key,
    s_writers_key: [lock_class_key, ..SB_FREEZE_LEVELS],

    i_lock_key: lock_class_key,
    i_mutex_key: lock_class_key,
    i_mutex_dir_key: lock_class_key
}

struct inode {
	i_mode: umode_t,
	i_opflags: u16,
	i_uid: kuid_t,
	i_gid: kgid_t,
	i_flags: uint,

#ifdef CONFIG_FS_POSIX_ACL
	i_acl: *mut posix_acl,
	i_default_acl: *mut posix_acl,
#endif

	i_op: *const inode_operations,
	i_sb: *mut super_block,
	i_mapping: *mut address_space,

#ifdef CONFIG_SECURITY
	i_security: *mut u8,
#endif

	/* Stat data, not accessed from path walking */
	i_ino: u64,
	/*
	 * Filesystems may only read i_nlink directly.  They shall use the
	 * following functions for modification:
	 *
	 *    (set|clear|inc|drop)_nlink
	 *    inode_(inc|dec)_link_count
	 */
    i_nlink: const uint,
	i_rdev: dev_t,
	i_size: loff_t,
	i_atime: *mut timespec,
	i_mtime: *mut timespec,
	i_ctime: *mut timespec,
	i_lock: spinlock_t, /* i_blocks, i_bytes, maybe i_size */
	i_bytes: u16,
	i_blkbits: uint,
	i_blocks: blkcnt_t,

#ifdef __NEED_I_SIZE_ORDERED
	i_size_seqcount: seqcount_t,
#endif

	/* Misc */
	i_state: unsigned long,
	i_mutex: *mut mutex,

	dirtied_when: unsigned long, /* jiffies of first dirtying */

	i_hash: *mut hlist_node,
	i_wb_list: *mut list_head, /* backing dev IO list */
	i_lru: *mut list_head, /* inode LRU list */
	i_sb_list: *mut list_head,
	union {
		struct hlist_head	i_dentry;
		struct rcu_head		i_rcu;
	};
	i_version: u64,
	i_count: atomic_t,
	i_dio_count: atomic_t,
	i_writecount: atomic_t,
	i_fop: *const file_operations, /* former ->i_op->default_file_ops */
	i_flock: *mut file_lock,
	i_data: *mut address_space,
#ifdef CONFIG_QUOTA
	i_dquot: [dquot, ..MAXQUOTAS],
#endif
	i_devices: *mut list_head,
	/* union {
		struct pipe_inode_info	*i_pipe;
		struct block_device	*i_bdev;
		struct cdev		*i_cdev;
	}; */
    i_bdev: *mut block_device;

	i_generation: u32,

#ifdef CONFIG_FSNOTIFY
	i_fsnotify_mask: __u32, /* all events this inode cares about */
	i_fsnotify_marks: *mut hlist_head,
#endif

#ifdef CONFIG_IMA
	i_readcount: atomic_t, /* struct files open RO */
#endif
	i_private: *mut	u8, /* fs or device private pointer */
};

extern {
    pub fn printk(fmt: *mut u8);
    pub fn kmalloc(size: uint) -> *mut u8;
    pub fn kfree(ptr: *mut u8);

    pub fn kill_litter_super(sb: *mut super_block);
    pub fn register_filesystem(fs: *mut file_system_type) -> int;
    pub fn unregister_filesystem(fs: *const file_system_type) -> int;
    pub fn mount_bdev(fs_type: *mut file_system_type,
                      flags: int,
                      dev: *const u8,
                      data: *mut u8,
                      fill_sb: fn(sb: *mut super_block,
                                  data: *mut u8,
                                  silent: int) -> int) -> *mut dentry;
    pub fn new_inode(sb: *mut super_block) -> *mut inode;
}

fn print(s: &str) {
    unsafe {
        let (ptr, _): (*mut u8, uint) = transmute(s);
        printk(ptr);
    }
}

fn IS_ERR<T>(ptr: *mut T) -> bool {
    unsafe {
        0xffffffffffffffffu64 - 4095u64 <= transmute(ptr)
    }
}

fn rustfs_fill_sb(sb: *mut super_block,
                  data: *mut u8,
                  silent: int)
    -> int
{
    -ENOMEM
}

fn rustfs_mount(fs_type: *mut file_system_type,
                flags: int,
                path: *const u8,
                data: *mut u8)
    -> *mut dentry
{
    unsafe {
        let entry = mount_bdev(fs_type, flags, path, data, rustfs_fill_sb);

        if IS_ERR(entry) {
            print("rustfs: mount failed\n");
        } else {
            print("rustfs: mounted successfully\n");
        }

        entry
    }
}

fn kill_litter_super_wrapper(sb: *mut super_block) {
    unsafe {
        kill_litter_super(sb);
    }
}

pub fn module_init(fs_type: *mut file_system_type) -> int {
    print("rustfs: hello\n");

    unsafe {
        (*fs_type).mount = rustfs_mount;
        (*fs_type).kill_sb = kill_litter_super_wrapper;

        return match register_filesystem(fs_type) {
            0 =>   { print("rustfs: successfully registered\n");     0 },
            err => { print("rustfs: register_filesystem failed\n"); -1 }
        }
    }
}

pub fn module_exit(fs_type: *const file_system_type) {
    print("rustfs: goodbye\n");

    unsafe {
        match unregister_filesystem(fs_type) {
            0 => print("rustfs: successfully unregistered\n"),
            err => print("rustfs: unregister_filesystem failed\n")
        }
    }
}

} // mod rustfs

#[no_mangle]
pub extern fn rustfs_module_init(fs_type: *mut rustfs::file_system_type) -> int {
    rustfs::module_init(fs_type)
}

#[no_mangle]
pub extern fn rustfs_module_exit(fs_type: *const rustfs::file_system_type) {
    rustfs::module_exit(fs_type)
}
