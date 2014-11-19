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
#include <net/cfg80211.h>

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

struct rfkill;

struct rfkill_ops {
	void	(*poll)(struct rfkill *rfkill, void *data);
	void	(*query)(struct rfkill *rfkill, void *data);
	int	(*set_block)(void *data, bool blocked);
};

struct cfg80211_registered_device {
	const struct cfg80211_ops *ops;
	struct list_head list;

	/* rfkill support */
	struct rfkill_ops rfkill_ops;
	struct rfkill *rfkill;
	struct work_struct rfkill_sync;

	/* ISO / IEC 3166 alpha2 for which this device is receiving
	 * country IEs on, this can help disregard country IEs from APs
	 * on the same alpha2 quickly. The alpha2 may differ from
	 * cfg80211_regdomain's alpha2 when an intersection has occurred.
	 * If the AP is reconfigured this can also be used to tell us if
	 * the country on the country IE changed. */
	char country_ie_alpha2[2];

	/* If a Country IE has been received this tells us the environment
	 * which its telling us its in. This defaults to ENVIRON_ANY */
	enum environment_cap env;

	/* wiphy index, internal only */
	int wiphy_idx;

	/* associated wireless interfaces, protected by rtnl or RCU */
	struct list_head wdev_list;
	int devlist_generation, wdev_id;
	int opencount; /* also protected by devlist_mtx */
	wait_queue_head_t dev_wait;

	struct list_head beacon_registrations;
	spinlock_t beacon_registrations_lock;

	/* protected by RTNL only */
	int num_running_ifaces;
	int num_running_monitor_ifaces;

	/* BSSes/scanning */
	spinlock_t bss_lock;
	struct list_head bss_list;
	struct rb_root bss_tree;
	u32 bss_generation;
	struct cfg80211_scan_request *scan_req; /* protected by RTNL */
	struct cfg80211_sched_scan_request *sched_scan_req;
	unsigned long suspend_at;
	struct work_struct scan_done_wk;
	struct work_struct sched_scan_results_wk;

#ifdef CONFIG_NL80211_TESTMODE
	struct genl_info *testmode_info;
#endif

	struct work_struct conn_work;
	struct work_struct event_work;

	struct delayed_work dfs_update_channels_wk;

	/* netlink port which started critical protocol (0 means not started) */
	u32 crit_proto_nlportid;

	struct cfg80211_coalesce *coalesce;

	/* must be last because of the way we do wiphy_priv(),
	 * and it should at least be aligned to NETDEV_ALIGN */
	struct wiphy wiphy __aligned(NETDEV_ALIGN);
};

static inline
struct cfg80211_registered_device *wiphy_to_rdev(struct wiphy *wiphy)
{
	return container_of(wiphy, struct cfg80211_registered_device, wiphy);
}

static struct wireless_dev *get_wdev_by_ifname(const char *ifname) {
    if (!ifname) {
        prerr("null ifname passed to get_wiphy_by_ifname");
        return NULL;
    }

    struct net *net = NULL;
    net = get_net_ns_by_pid(1); /* TODO */
    if (!net) {
        prerr("get_ns_by_pid failed");
        return NULL;
    }

    struct net_device *dev = NULL;
    dev = dev_get_by_name(net, ifname);
    if (!dev) {
        prerr("dev_get_by_name failed");
        return NULL;
    }

    return dev->ieee80211_ptr;
}

static int count_channels(const struct wiphy *wiphy) {
    if (!wiphy) {
        prerr("null wiphy passed to count_channels");
        return 0;
    }

    enum ieee80211_band band;
    int n_channels = 0;

    for (band = 0; band < IEEE80211_NUM_BANDS; band++) {
        if (wiphy->bands[band]) {
            n_channels += wiphy->bands[band]->n_channels;
        }
    }

    return n_channels;
}

static void scan_req_fill_channels(struct cfg80211_scan_request *out_req,
                                   struct wiphy *wiphy) {
    if (!out_req || !wiphy) {
        prerr("invalid arguments to scan_req_fill_channels: out_req = %p, "
              "wiphy = %p", out_req, wiphy);
        return;
    }

    enum ieee80211_band band;
    int i = 0;

    /* all channels */
    for (band = 0; band < IEEE80211_NUM_BANDS; band++) {
        int j;

        if (!wiphy->bands[band]) {
            continue;
        }

        for (j = 0; j < wiphy->bands[band]->n_channels; j++) {
            struct ieee80211_channel *chan;

            chan = &wiphy->bands[band]->channels[j];

            if (chan->flags & IEEE80211_CHAN_DISABLED)
                continue;

            out_req->channels[i] = chan;
            i++;
        }
    }

    out_req->n_channels = i;
}

static struct cfg80211_scan_request *create_scan_req(struct wireless_dev *wdev) {
    if (!wdev || !wdev->wiphy) {
        prerr("invalid arguments to count_channels: wdev = %p, wdev->wiphy = %p",
              wdev, wdev->wiphy);
        return NULL;
    }

    int n_channels = count_channels(wdev->wiphy);
    if (n_channels <= 0) {
        prerr("invalid channel count: %d", n_channels);
        return NULL;
    }

    struct cfg80211_scan_request *req =
            kzalloc(sizeof(*req) + n_channels * sizeof(*req->channels),
                    GFP_KERNEL);

    scan_req_fill_channels(req, wdev->wiphy);

    req->wdev = wdev;
    req->wiphy = wdev->wiphy;
    req->notified = true;

    return req;
}

static int __init simple_init(void) {
    prinfo("init");

    struct wireless_dev *wdev = get_wdev_by_ifname("wlan0");
    if (!wdev) {
        prerr("ieee80211_ptr null");
        return -1;
    }
    if (!wdev->wiphy) {
        prerr("wiphy null");
        return -1;
    }

    struct cfg80211_registered_device *rdev = wiphy_to_rdev(wdev->wiphy);
    if (!rdev) {
        prerr("rdev null");
        return -1;
    }
    if (rdev->scan_req) {
        prerr("scan_req not null");
        return -1;
    }

    const struct cfg80211_ops *ops = rdev->ops;
    if (!ops) {
        prerr("ops null");
        return -1;
    }

    struct cfg80211_scan_request *req = create_scan_req(wdev);
    if (!req) {
        prerr("cannot create scan_req");
        return -1;
    }

    rdev->scan_req = req;

    int result = ops->scan(&rdev->wiphy, req);
    if (result) {
        prerr("scan returned %d", result);
        goto cleanup;
    }

    int i;
    for (i = 0; i < req->n_ssids; ++i) {
        const struct cfg80211_ssid *ssid = &req->ssids[i];

        prinfo("%d: %.*s", i + 1, (int)ssid->ssid_len, ssid->ssid);
    }

cleanup:
    rdev->scan_req = NULL;
    kfree(req);
    return 0;
}

static void __exit simple_exit(void) {
    prinfo("exit");
}

module_init(simple_init);
module_exit(simple_exit);
