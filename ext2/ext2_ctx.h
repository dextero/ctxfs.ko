#ifndef EXT2_CTX_H
#define EXT2_CTX_H

#include <net/cfg80211.h>

struct ext2_inode_info;

extern int ext2_ctx_get_curr_ssid(struct cfg80211_ssid *out_ssid);

extern int ext2_ctx_find_root_ino(struct super_block *sb,
                                  int *out_ino,
                                  int orig_root_ino);

extern int ext2_ctx_adjust_root(struct super_block *sb);

#endif // EXT2_CTX_H
