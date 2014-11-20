#include <linux/module.h>
#include <linux/netdevice.h>
#include <net/cfg80211.h>
#include <net/iw_handler.h>

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

static int get_essid_for_dev(struct net_device *netdev,
                             struct wiphy *wiphy,
                             char out_essid[IEEE80211_MAX_SSID_LEN]) {
    struct iw_request_info info;
    union iwreq_data wrqu;
    iw_handler get_essid =
            (iw_handler)wiphy->wext->standard[IW_IOCTL_IDX(SIOCGIWESSID)];

    memset(&info, 0, sizeof(info));
    memset(&wrqu, 0, sizeof(wrqu));
    memcpy(wrqu.name, netdev->name, sizeof(wrqu.name));
    wrqu.essid.pointer = out_essid;
    wrqu.essid.length = IEEE80211_MAX_SSID_LEN;

    int result = get_essid(netdev, &info, &wrqu, out_essid);
    if (result < 0) {
        return result;
    }

    return wrqu.essid.length;
}

static int get_connected_ssids(struct cfg80211_ssid ssids[],
                               int max_ssids) {
    int ssid_idx = 0;
    struct net_device *dev = first_net_device(&init_net);

    for (; dev && ssid_idx < max_ssids;
           dev = next_net_device(dev)) {
        if (!dev->ieee80211_ptr || !dev->ieee80211_ptr->wiphy) {
            prinfo("%.*s: not a wireless device", IFNAMSIZ, dev->name);
            continue;
        }

        struct wiphy *wiphy = dev->ieee80211_ptr->wiphy;

        int result = get_essid_for_dev(dev, wiphy, ssids[ssid_idx].ssid);
        if (result < 0) {
            prerr("%.*s: get_essid_for_dev returned %d", IFNAMSIZ, dev->name, result);
            continue;
        }

        if (result == 0) {
            prinfo("%.*s: not connected", IFNAMSIZ, dev->name);
            continue;
        }

        ssids[ssid_idx].ssid_len = result;
        prinfo("%.*s: connected to %.*s", IFNAMSIZ, dev->name,
               (int)ssids[ssid_idx].ssid_len, ssids[ssid_idx].ssid);

        ++ssid_idx;
    }

    return ssid_idx;
}

static int __init simple_init(void) {
    static const int MAX_SSIDS = 8;

    prinfo("init");

    struct cfg80211_ssid connected_ssids[MAX_SSIDS];
    int num_ssids = get_connected_ssids(connected_ssids,
                                        ARRAY_SIZE(connected_ssids));
    if (num_ssids < 0) {
        prerr("cannot get SSIDS");
        return num_ssids;
    }

    prinfo("connected to %d networks", num_ssids);
    return 0;
}

static void __exit simple_exit(void) {
    prinfo("exit");
}

module_init(simple_init);
module_exit(simple_exit);
