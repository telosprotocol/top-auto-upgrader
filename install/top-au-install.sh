#!/usr/bin/env bash
PATH=/bin:/sbin:/usr/bin:/usr/sbin:/usr/local/bin:/usr/local/sbin:~/bin
export PATH

#=================================================================#
#   System Required:  CentOS 7, Debian, Ubuntu                    #
#   Description: Script of Install TOP Auto Upgrader Server       #
#   Author: Charles                                               #
#=================================================================#

proj_name="TOP Auto Upgrader"

# TODO change after repo move to telosprotocol
daemon_script_url="https://raw.githubusercontent.com/CharlesLiu-TOPNetwork/top-auto-upgrader/master/install/top-au-daemon.sh"

target_dir=/usr/bin
config_dir=/etc/top-au
service_dir=/lib/systemd/system
service_name=top-au
service_stub=/etc/init.d/${service_name}
bin_name=top-auto-upgrader


# bool result
cmd_success=0
cmd_failed=1

# Color
red='\033[0;31m'
green='\033[0;32m'
yellow='\033[0;33m'
plain='\033[0m'

#Current folder
cur_dir=`pwd`

# Check system
function check_sys() {
    local checkType=${1}
    local value=${2}

    local release=''
    local systemPackage=''

    if [[ -f /etc/redhat-release ]]; then
        release="centos"
        systemPackage="yum"
    elif grep -Eqi "debian" /etc/issue; then
        release="debian"
        systemPackage="apt"
    elif grep -Eqi "ubuntu" /etc/issue; then
        release="ubuntu"
        systemPackage="apt"
    elif grep -Eqi "centos|red hat|redhat" /etc/issue; then
        release="centos"
        systemPackage="yum"
    elif grep -Eqi "debian" /proc/version; then
        release="debian"
        systemPackage="apt"
    elif grep -Eqi "ubuntu" /proc/version; then
        release="ubuntu"
        systemPackage="apt"
    elif grep -Eqi "centos|red hat|redhat" /proc/version; then
        release="centos"
        systemPackage="yum"
    fi

    if [[ "${checkType}" == "sysRelease" ]]; then
        if [ "${value}" == "${release}" ]; then
            return "${cmd_success}"
        else
            return "${cmd_failed}"
        fi
    elif [[ "${checkType}" == "packageManager" ]]; then
        if [ "${value}" == "${systemPackage}" ]; then
            return "${cmd_success}"
        else
            return "${cmd_failed}"
        fi
    fi
}

function get_char() {
    SAVEDSTTY=`stty -g`
    stty -echo
    stty cbreak
    dd if=/dev/tty bs=1 count=1 2> /dev/null
    stty -raw
    stty echo
    stty $SAVEDSTTY
}

# write binary config file
function write_top_auto_upgrader_config() {
    [ ! -d ${config_dir} ] && mkdir ${config_dir}

    cat > ${config_dir}/config.json <<-EOF
{
    "__test": "__reserved",
}
EOF
}

# write service description file to /lib/systemd/system/
function write_service_description_file() {
    local svc_name=${1}
    local svc_stub=${2}

    cat > "${service_dir}"/"${svc_name}".service <<-EOF
[Unit]
    Description=${svc_name}
    After=network.target
[Service]
    Type=forking
    ExecStart=${svc_stub} start
    ExecReload=${svc_stub} restart
    ExecStop=${svc_stub} stop
    PrivateTmp=true
    Restart=on-failure
    RestartSec=30s
    LimitNOFILE=1000000
    LimitCORE=infinity
[Install]
    WantedBy=multi-user.target
EOF

    chmod 754 "${service_dir}"/"${svc_name}".service
}

# Install top_au service
function install_top_au_service() {
    # ldconfig  # we might not need this , if make musl binary. Later check.

    cd ${cur_dir}

}

function build_or_fetch_top_auto_upgrader() {
    # TODO change from
    # test only
    /bin/cp -rfa ./target/debug/${bin_name} ${target_dir}

    cd ${cur_dir}
    if [ -f ${target_dir}/${bin_name} ]; then
        # Download service daemon script

        if ! curl -L ${daemon_script_url} -o ${service_stub} ; then
            echo -e "[${red}Error${plain}] Failed to download ${proj_name} chkconfig file!"
            exit 1
        fi

        chmod +x ${service_stub}
        if check_sys packageManager yum; then
            chkconfig --add ${service_name}
            chkconfig ${service_name} on
        elif check_sys packageManager apt; then
            update-rc.d -f ${service_name} defaults
        fi

        write_service_description_file ${service_name} ${service_stub}

        # ${service_stub} start
        systemctl enable ${service_name}.service
        systemctl start ${service_name}.service

        # TODO add some success infomations.
    else
        echo "${proj_name} install failed, please contact @top"
        exit 1
    fi
}


function do_uninstall_action() {
    ${service_stub} status > /dev/null 2>&1
    if [ $? -eq 0 ]; then
        ${service_stub} stop
    fi
    if check_sys packageManager yum; then
        chkconfig --del "${service_name}"
    elif check_sys packageManager apt; then
        update-rc.d -f "${service_name}" remove
    fi

    systemctl stop "${service_name}".service

    rm -rf "${config_dir}"
    rm -f "${service_stub}"
    rm -f "${target_dir}"/"${bin_name}"
    rm -f "${service_dir}"/"${service_name}".service
    echo "${proj_name} uninstall success!"
}

# Uninstall
function __uninstall() {
    printf "Are you sure uninstall ${proj_name}? (y/n)\n"
    read -p "(Default: n):" answer
    [ -z "${answer}" ] && answer="n"
    if [ "${answer}" == "y" ] || [ "${answer}" == "Y" ]; then
        do_uninstall_action
    else
        echo
        echo "uninstall cancelled, nothing to do..."
        echo
    fi
}

# Install
function __install() {
    
    # 0. disable se linux
    # 1. pre install if need, ask config that need to fill into json.

    # 2. echo double check ready
    echo
    echo "Press any key to start...or Press Ctrl+C to cancel"
    char=`get_char`

    cd ${cur_dir}

    # 3. install neccessary build tools

    # 4. uninstall old auto upgrader
    do_uninstall_action

    # 5. build or fetch new one
    build_or_fetch_top_auto_upgrader

    # 6. write config
    write_top_auto_upgrader_config

    # 7. install service


    echo ""
}


function main() {
    # clear
    echo
    echo "# Script of Install ${proj_name} Server"
    echo "######################################################################"
    echo "# Author: Charles                                                    #"
    echo "# Github: https://github.com/CharlesLiu-TOPNetwork/top-auto-upgrader #"
    echo "######################################################################"
    echo

    # Make sure only root can run our script
    [[ $EUID -ne 0 ]] && echo -e "[${red}Error${plain}] This script must be run as root!" && exit 1

    # Initialization step
    local action=$1
    [ -z "$1" ] && action=install
    case "${action}" in
        install|uninstall)
            __"${action}"
            ;;
        *)
            echo "Arguments error! [${action}]"
            echo "Usage: `basename "$0"` [install|uninstall]"
            ;;
    esac

    exit 0
}


#=================================================================#
#                    script begin entry                           #
#=================================================================#

main "$1"