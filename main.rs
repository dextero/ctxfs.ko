#![allow(ctypes)]
#![no_std]
#![feature(intrinsics)]
#![feature(lang_items)]

#[lang="sized"]
trait Sized {}

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

extern {
    pub fn printk(fmt: *mut u8);

    pub fn kill_litter_super(sb: *mut super_block);
    pub fn register_filesystem(fs: *mut file_system_type) -> int;
    pub fn unregister_filesystem(fs: *const file_system_type);
    pub fn mount_bdev(fs_type: *mut file_system_type,
                      flags: int,
                      dev: *const u8,
                      data: *mut u8,
                      fill_sb: fn(sb: *mut super_block,
                                  data: *mut u8,
                                  silent: int) -> int) -> *mut dentry;
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
            print("cannnot mount rustfs\n");
        } else {
            print("rustfs mounted successfully\n");
        }

        entry
    }
}

fn kill_litter_super_wrapper(sb: *mut super_block) {
    unsafe {
        kill_litter_super(sb);
    }
}

#[no_mangle]
pub extern fn rustfs_module_init(fs_type: *mut file_system_type) -> int {
    unsafe {
        print("hello\n");

        (*fs_type).mount = rustfs_mount;
        (*fs_type).kill_sb = kill_litter_super_wrapper;

        if register_filesystem(fs_type) != 0 {
            return -1;
        }
    }

    return 0;
}

#[no_mangle]
pub extern fn rustfs_module_exit(fs_type: *const file_system_type) {
    unsafe {
        print("goodbye\n");

        unregister_filesystem(fs_type);
    }
}
