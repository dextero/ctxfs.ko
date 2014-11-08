#![allow(ctypes)]
#![no_std]
#![feature(intrinsics)]
#![feature(lang_items)]
#![feature(globs)]

#[lang="sized"] trait Sized {}
#[lang="copy"] trait Copy {}
#[lang="fail"] fn fail(expr_file_line: &(&'static str, &'static str, uint)) -> ! { loop {} }
#[lang="fail_bounds_check"] fn fail_bounds_check(file_line: &(&'static str, uint), index: uint, len: uint) -> ! { loop {} }

mod rustfs {

extern "rust-intrinsic" {
    pub fn transmute<T, U>(val: T) -> U;
}

const RUSTFS_MAGIC_NUMBER: u64 = 0x72757374u64;

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

const S_IFMT:   u16 = 0170000;
const S_IFSOCK: u16 = 0140000;
const S_IFLNK:  u16 = 0120000;
const S_IFREG:  u16 = 0100000;
const S_IFBLK:  u16 = 0060000;
const S_IFDIR:  u16 = 0040000;
const S_IFCHR:  u16 = 0020000;
const S_IFIFO:  u16 = 0010000;
const S_ISUID:  u16 = 0004000;
const S_ISGID:  u16 = 0002000;
const S_ISVTX:  u16 = 0001000;

type umode_t = u16;
type kuid_t = u32; // actually a struct containing u32
type kgid_t = u32; // actually a struct containing u32
type dev_t = u32;
type loff_t = i64;
type blkcnt_t = u64;
type atomic_t = i32;

// stubs
pub struct address_space;
pub struct backing_dev_info;
pub struct block_device;
pub struct dentry;
pub struct dentry_operations;
pub struct dquot_operations;
pub struct export_operations;
pub struct file_lock;
pub struct file_operations;
pub struct hlist_bl_head;
pub struct hlist_head;
pub struct hlist_node;
pub struct inode_operations;
pub struct list_head;
pub struct list_lru;
pub struct lock_class_key;
pub struct module;
pub struct mtd_info;
pub struct mutex;
pub struct quota_info;
pub struct quotactl_ops;
pub struct rcu_head;
pub struct rw_semaphore;
pub struct sb_writers;
pub struct shrinker;
pub struct super_operations;
pub struct workqueue_struct;
pub struct xattr_handler;

#[repr(C)]
struct spinlock_t {
    lock: u32, // actually, arch_spinlock_t
#ifdef CONFIG_GENERIC_LOCKBREAK
	break_lock: u32
#endif
    // screew debug stuff
/*
#ifdef CONFIG_DEBUG_SPINLOCK
	unsigned int magic, owner_cpu;
	void *owner;
#endif
#ifdef CONFIG_DEBUG_LOCK_ALLOC
	struct lockdep_map dep_map;
#endif
*/
}

#[repr(C)]
pub struct callback_head {
	next: *mut callback_head,
	head: fn(head: *mut callback_head)
}

#[repr(C)]
pub struct file_system_type {
    name: *const u8,
    fs_flags: int,

    mount: fn(fs: &mut file_system_type,
              flags: int,
              path: *const u8,
              data: *mut u8) -> *mut dentry,
    kill_sb: fn(sb: &mut super_block),

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

#[repr(C)]
pub struct super_block {
    list_head: list_head,
    s_list: list_head,
    s_dev: int,
    s_blocksize_bits: u8,
    s_blocksize: u64,
    s_maxbytes: int,
    file_system_type: file_system_type,
    s_type: *mut file_system_type,
    super_operations: super_operations,
    s_op: *const super_operations,
    dquot_operations: dquot_operations,
    dq_op: *const dquot_operations,
    quotactl_ops: quotactl_ops,
    s_qcop: *const quotactl_ops,
    export_operations: export_operations,
    s_export_op: *const export_operations,
    s_flags: u64,
    s_magic: u64,
    dentry: dentry,
    s_root: *mut dentry,
    rw_semaphore: rw_semaphore,
    s_umount: rw_semaphore,
    s_count: int,
    s_active: int,
#ifdef CONFIG_SECURITY
    s_security: *mut (),
#endif
    xattr_handler: xattr_handler,
    s_xattr: *mut *const xattr_handler,
    s_inodes: list_head,
    hlist_bl_head: hlist_bl_head,
    s_anon: hlist_bl_head,
    s_mounts: list_head,
    block_device: block_device,
    s_bdev: *mut block_device,
    backing_dev_info: backing_dev_info,
    s_bdi: *mut backing_dev_info,
    mtd_info: mtd_info,
    s_mtd: *mut mtd_info,
    hlist_node: hlist_node,
    s_instances: hlist_node,
    quota_info: quota_info,
    s_dquot: quota_info,
    sb_writers: sb_writers,
    s_writers: sb_writers,
    s_id: [i8, ..32],
    s_uuid: [int, ..16],
    s_fs_info: *mut (),
    s_max_links: uint,
    s_mode: int,
    s_time_gran: int,
    mutex: mutex,
    s_vfs_rename_mutex: mutex,
    s_subtype: *mut i8,
    __rcu: i8,
    dentry_operations: dentry_operations,
    s_d_op: *const dentry_operations,
    cleancache_poolid: int,
    shrinker: shrinker,
    s_shrink: shrinker,
    s_remove_count: int,
    s_readonly_remount: int,
    workqueue_struct: workqueue_struct,
    s_dio_done_wq: *mut workqueue_struct,
    list_lru: list_lru,
    s_dentry_lru: list_lru,
    s_inode_lru: list_lru,
    rcu_head: rcu_head,
    rcu: rcu_head,
    s_stack_depth: int
}

#[repr(C)]
struct timespec {
    ts_sec: i64,
    ts_nsec: i64
}

#[repr(C)]
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
    i_nlink: uint,
	i_rdev: dev_t,
	i_size: loff_t,
	i_atime: timespec,
	i_mtime: timespec,
	i_ctime: timespec,
	i_lock: spinlock_t, /* i_blocks, i_bytes, maybe i_size */
	i_bytes: u16,
	i_blkbits: uint,
	i_blocks: blkcnt_t,

#ifdef __NEED_I_SIZE_ORDERED
	i_size_seqcount: seqcount_t,
#endif

	/* Misc */
	i_state: u64,
	i_mutex: mutex,

	dirtied_when: u64, /* jiffies of first dirtying */

	i_hash: hlist_node,
	i_wb_list: list_head, /* backing dev IO list */
	i_lru: list_head, /* inode LRU list */
	i_sb_list: list_head,

	/* union {
		struct hlist_head	i_dentry;
		struct rcu_head		i_rcu;
	}; */
    i_rcu: callback_head,

	i_version: u64,
	i_count: atomic_t,
	i_dio_count: atomic_t,
	i_writecount: atomic_t,
	i_fop: *const file_operations, /* former ->i_op->default_file_ops */
	i_flock: *mut file_lock,
	i_data: *mut address_space,
#ifdef CONFIG_QUOTA
	i_dquot: [*mut dquot, ..MAXQUOTAS],
#endif
	i_devices: list_head,

	/* union {
		struct pipe_inode_info	*i_pipe;
		struct block_device	*i_bdev;
		struct cdev		*i_cdev;
	}; */
    i_bdev: *mut block_device,

	i_generation: u32,

#ifdef CONFIG_FSNOTIFY
	i_fsnotify_mask: u32, /* all events this inode cares about */
	i_fsnotify_marks: *mut hlist_head,
#endif

#ifdef CONFIG_IMA
	i_readcount: atomic_t, /* struct files open RO */
#endif
	i_private: *mut	u8, /* fs or device private pointer */
}

type c_str = *const u8;

extern {
    pub fn printk(fmt: *mut u8);
    //pub fn kmalloc(size: uint) -> *mut u8;
    //pub fn kfree(ptr: *mut u8);

    pub fn kill_litter_super(sb: *mut super_block);
    pub fn register_filesystem(fs: *mut file_system_type) -> int;
    pub fn unregister_filesystem(fs: *const file_system_type) -> int;
    pub fn mount_bdev(fs_type: *mut file_system_type,
                      flags: int,
                      dev: c_str,
                      data: *mut u8,
                      fill_sb: fn(sb: *mut super_block,
                                  data: *mut u8,
                                  silent: int) -> int) -> *mut dentry;
    pub fn new_inode(sb: *mut super_block) -> *mut inode;
    pub fn inode_init_owner(inode: *mut inode,
                            dir: *mut inode,
                            mode: umode_t);
}

fn print(s: &str) {
    unsafe {
        let (ptr, _): (*mut u8, uint) = transmute(s);
        printk(ptr);
    }
}

fn sprinti(num: i64,
           buf: &mut [u8, ..32]) -> uint {
    print("sprinti 0\n");
    let div10 = num / 10;
    print("sprinti 1\n");
    let mod10 = (num % 10) as u8;

    print("sprinti 2\n");
    match (div10) {
        0 => {
            buf[0] = mod10;
            1
        },
        n => {
            let idx = sprinti(n, buf);
            buf[idx] = '0' as u8 + mod10;
            idx + 1
        }
    }
}

fn dupa() -> uint {
    print("dupa\n");
    0
}

fn printi(num: i64) {
    print("printi 0\n");
    let mut buf = ['\0', ..32];
    print("printi 1\n");
    let idx = dupa(); //sprinti(num, &mut buf);
    print("printi 2\n");
    buf[idx] = '\n';
    print("printi 3\n");
    buf[idx + 1] = '\0';

    print("printi 4\n");
    //unsafe {
        //printk(transmute(&mut buf))
    //}
}

//fn print2<T>(s: &str, a1: &T) {
//    unsafe {
//        let (ptr, _): (*mut u8, uint) = transmute(s);
//        let _printk: fn(*mut u8, T) = transmute(printk);
//        _printk(ptr, *a1);
//    }
//}

fn nullptr<T>() -> *mut T {
    unsafe {
        transmute(0u)
    }
}

fn to_ref<T>(ptr: *mut T) -> &'static mut T {
    unsafe {
        transmute(ptr)
    }
}

fn from_ref<T>(ptr: &T) -> *mut T {
    unsafe {
        transmute(ptr)
    }
}

fn is_null<T>(r: &T) -> bool {
    unsafe {
        from_ref(r) == nullptr()
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
    let root: &mut inode;

    unsafe {
        root = to_ref(new_inode(sb));

        if is_null(root) {
            print("rustfs: new_inode failed\n");
            return -1;
        }

    (*sb).s_magic = RUSTFS_MAGIC_NUMBER;
    //(*sb).s_op =

    root.i_ino = 0;
    unsafe {
        root.i_sb = transmute(sb);
    }
    root.i_atime = timespec { ts_sec: 0, ts_nsec: 0 };
    root.i_mtime = timespec { ts_sec: 0, ts_nsec: 0 };
    root.i_ctime = timespec { ts_sec: 0, ts_nsec: 0 };

        inode_init_owner(root, nullptr(), S_IFDIR);
    }

    -ENOMEM
}

fn rustfs_mount(fs_type: &mut file_system_type,
                flags: int,
                path: *const u8,
                data: *mut u8)
    -> *mut dentry
{
    print("rustfs: mount\n");

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

fn kill_litter_super_wrapper(sb: &mut super_block) {
    print("kill_litter_super_wrapper\n");

    unsafe {
        kill_litter_super(sb);
    }
}

pub fn module_init(fs_type: &mut file_system_type) -> int {
    print("rustfs: hello\n");

    unsafe {
        fs_type.mount = rustfs_mount;
        fs_type.kill_sb = kill_litter_super_wrapper;

        print("rustfs: module_init/printi\n");
        printi(1234i64);
        print("rustfs: register_filesystem\n");
        let err = register_filesystem(fs_type);
        print("rustfs: returned from register_filesystem\n");
        printi(err as i64);
        match err {
            0 => { print("rustfs: successfully registered\n"); 0 },
            err => {
                print("rustfs: register_filesystem failed\n");
                printi(err as i64);
                -1
            }
        }
    }
}

pub fn module_exit(fs_type: &file_system_type) {
    print("rustfs: goodbye\n");

    unsafe {
        match unregister_filesystem(fs_type) {
            0 => print("rustfs: successfully unregistered\n"),
            err => {
                print("rustfs: unregister_filesystem failed\n");
                printi(err as i64)
            }
        };
    }
}

pub fn printi_test() -> int {
    print("before\n");
    printi(1234i64);
    print("after\n");
    0
}

} // mod rustfs

#[no_mangle]
pub extern "C" fn rustfs_module_init(fs_type: &mut rustfs::file_system_type) -> int {
    //rustfs::module_init(fs_type)
    rustfs::printi_test()
}

#[no_mangle]
pub extern "C" fn rustfs_module_exit(fs_type: &rustfs::file_system_type) {
    rustfs::module_exit(fs_type)
}
