# TOP Auto Upgrader

Auto Upgrade Service for [TOP-Chain](https://github.com/telosprotocol/TOP-chain)

## TOPIO Needed

Before using this auto-upgrader, please make sure you have install topio and had workable mining account. 

As the auto upgrader only handle the `upgrade` part, it can not do these prepare work for you.

Install topio reference here: https://developers.topnetwork.org/Tools/TOPIO/Overview/

If you have finished install topio, move on.

## Install

``` BASH
wget --no-check-certificate https://raw.githubusercontent.com/telosprotocol/top-auto-upgrader/master/install/top-au-install.sh
chmod +x top-au-install.sh
sudo ./top-au-install.sh
```

Then follow the interactive script, it will collect infomation that auto-upgrader must need, like your topio directory and mining account.

If you use root user to start topio before, it could be quite simple, just input the keystore's password, and press some `Enter` to start installation.


## FAQs

### system support

* ✔ centos7-centos9
* ✔ ubuntu18-ubuntu22

<!-- We would recommand to use Centos7 anyway. -->



### Root User Needed

Install Top-Auto-Upgrader Service need sudo permission.

While it do support any user with sudo permission, it would be less trouble if you installed topio with root user.


## Control top-au Service

### Status

use below command to show service status:

``` BASH
systemctl status top-au
```

After well-installed, it should print something like these in you console:

``` BASH

● top-au.service - top-au
     Loaded: loaded (/lib/systemd/system/top-au.service; enabled; vendor preset: enabled)
     Active: active (exited) since Wed 2023-01-04 07:23:04 UTC; 20s ago
   Main PID: 20710 (code=exited, status=0/SUCCESS)
      Tasks: 10 (limit: 1116)
     Memory: 269.7M
        CPU: 12.903s
     CGroup: /system.slice/top-au.service
             ├─20711 /usr/bin/top-auto-upgrader -d -c /etc/top-au/config.json
             ├─20949 "topio: node safebox process"
             ├─20971 "topio: daemon process node startNode"
             └─20972 "topio: xnode process node startNode"
```

### Stop/Start/Restart

Stop Service with command:

``` BASH
systemctl stop top-au
```

Start Service with command:

``` BASH
systemctl start top-au
```

Restart Service with command:

``` BASH
systemctl restart top-au
```