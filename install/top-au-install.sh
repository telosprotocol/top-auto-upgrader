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


# config_var:
config_machine_id=""
config_topio_home_dir=""
config_topio_package_dir="/"
config_topio_mining_keystore_file=""
config_topio_mining_pub_key=""
config_topio_mining_key_pswd=""
config_topio_user=""

# bool result
cmd_success=0
cmd_failed=1

# Color
red='\033[0;31m'
green='\033[0;32m'
yellow='\033[0;33m'
plain='\033[0m'

#Current folder
cur_dir=$(pwd)

login_user=$(eval logname)
login_user_home_dir=$( getent passwd ${login_user} | cut -d: -f6 )
# echo "$login_user"
# echo "$login_user_home_dir"

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
    SAVEDSTTY=$(stty -g)
    stty -echo
    stty cbreak
    dd if=/dev/tty bs=1 count=1 2> /dev/null
    stty -raw
    stty echo
    stty $SAVEDSTTY
}

function _pre_install_machine_id() {
    # get machine_id
    config_machine_id=$(cat /etc/machine-id)
    echo
    echo "----------------------------------------------------------------"
    echo "machine-id: ${config_machine_id}"
    echo "----------------------------------------------------------------"
    echo
}

function _pre_quite_check_package_dir() {
    # default dir is login_user_home_dir
    
    # check default dir exist
    [ ! -d "${login_user_home_dir}" ] && return ${cmd_failed}

    cd ${login_user_home_dir}
    local latest_version=$(find ./ -maxdepth 1 -type d -name "topio-*-release" | awk -F '-' '{print $2}' | sort -V | tail -n 1)
    [ -z "${latest_version}" ] && cd ${cur_dir} && return ${cmd_failed}
    cd ${cur_dir}

    config_topio_package_dir=${login_user_home_dir}
    
    echo "Find: package directory: ${config_topio_package_dir}"
    echo "Find: latest topio version: ${latest_version}"

    return ${cmd_success}
}

function _pre_install_package_dir() {
    # get TOPIO_package_dir
    echo -e "${green}Please Input current TOPIO release package directory ( Default value is ${login_user_home_dir} ) :${plain}"
    while true; do
        read -p "(Please Input):" config_topio_package_dir
        # check input empty
        [ -z "${config_topio_package_dir}" ] && config_topio_package_dir="${login_user_home_dir}"
        # check dir exist
        [ ! -d "${config_topio_package_dir}" ] && echo -e "[${red}Error${plain}]Directory ${config_topio_package_dir} not exist" && echo && continue

        # find topio-version-release:
        cd ${config_topio_package_dir}
        local latest_version=$(find ./ -maxdepth 1 -type d -name "topio-*-release" | awk -F '-' '{print $2}' | sort -V | tail -n 1)
        [ -z "${latest_version}" ] && echo -e "[${red}Error${plain}]Not find any topio packet at ${config_topio_package_dir}, consider install topio first." && continue

        echo "find latest topio version: ${latest_version}"
        cd ${cur_dir}
        break
    done;

    echo
    echo "----------------------------------------------------------------"
    echo "topio release package save in: ${config_topio_package_dir}"
    echo "----------------------------------------------------------------"
    echo
}

function _pre_quite_check_data_dir() {
    # default data dir is ~/topnetwork
    local default_topio_home_dir="${login_user_home_dir}/topnetwork"

    # check default dir exist
    [ ! -d "${default_topio_home_dir}" ] && return ${cmd_failed}

    # check dir/keystore exist
    [ ! -d "${default_topio_home_dir}/keystore" ] && return ${cmd_failed}

    config_topio_home_dir=${default_topio_home_dir}

    echo "Find: topio home directory: ${config_topio_home_dir}"

    return ${cmd_success}
}

function _pre_install_data_dir() {
    # get TOPIO_data_dir
    local default_topio_home_dir="${login_user_home_dir}/topnetwork"
    echo -e "${green}Please Input current TOPIO data directory ( Default value is ${default_topio_home_dir} ) :${plain}"
    config_topio_home_dir=$(printenv TOPIO_HOME )
    [ -n "${config_topio_home_dir}" ] && echo "(Detect environment variable): ${config_topio_home_dir}"
    
    while true; do
        read -p "(Please Input):" config_topio_home_dir
        # check input empty
        [ -z "${config_topio_home_dir}" ] && config_topio_home_dir="${default_topio_home_dir}"
        # check dir exist
        [ ! -d "${config_topio_home_dir}" ] && echo -e "[${red}Error${plain}]Directory ${config_topio_home_dir} not exist!" && echo && continue
        # check dir/keystore exist
        [ ! -d "${config_topio_home_dir}/keystore" ] && echo -e "[${red}Error${plain}]Directory ${config_topio_home_dir}/keystore not exist! " && echo && continue
        break
    done;

    echo
    echo "----------------------------------------------------------------"
    echo "TOPIO_HOME: ${config_topio_home_dir}"
    echo "----------------------------------------------------------------"
    echo
}

function _pre_quite_check_mining_key() {
    # if only one keystore account find.
    cd ${config_topio_home_dir}
    [ $( find ./keystore -type f | wc -l ) -ne 1 ] && cd ${cur_dir} && return ${cmd_failed}
    [ $( grep "public_key" ./keystore/* | wc -l ) -ne 1 ] && cd ${cur_dir} && return ${cmd_failed}
    config_topio_mining_pub_key=$( grep "public_key" ./keystore/* | grep ": \".*\"" -oE | sed 's/[:\"[:blank:]]*//g' )
    config_topio_mining_keystore_file=$( find ./keystore/* | xargs readlink -f )

    echo "Find: keystore file is ${config_topio_mining_keystore_file}"
    echo "Find: public key is ${config_topio_mining_pub_key}"

    return ${cmd_success}
}

function _pre_install_mining_key() {
    # get default MinerAddress
    echo -e "${green}Please Input the account public key used for mining:${plain}"
    cd ${config_topio_home_dir}
    echo 
    [ $( find ./keystore -type f | wc -l ) -ne 0 ] &&  echo "It must be one of these:" && echo && find ./keystore -type f | xargs grep "pub" && echo 

    while true; do
        read -p "(Please Input):" config_topio_mining_pub_key
        # check if public key exist in one of keystore.
        [ $( grep "\"${config_topio_mining_pub_key}\"" ./keystore/* | wc -l ) -ne 1 ] && echo -e "[${red}Error${plain}] can not find corresponding keystore file of ${config_topio_mining_pub_key}" && echo && continue
        config_topio_mining_keystore_file=$( grep "\"${config_topio_mining_pub_key}\"" ./keystore/* -l | xargs readlink -f )
        break;
    done;
    cd ${cur_dir}

    echo
    echo "----------------------------------------------------------------"
    echo "Mining Key Keystore: ${config_topio_mining_keystore_file}"
    echo "Mining Public Key: ${config_topio_mining_pub_key}"
    echo "----------------------------------------------------------------"
    echo
}

function _pre_install_mining_key_pswd() {
    # get pswd of mining key
    echo -e "${green}Please Input the account public key's password:${plain}"
    while true; do
        read -p "(Please Input):" config_topio_mining_key_pswd
        local mining_key_pswd_check=""
        read -p "(Please Input Twice for double check):" mining_key_pswd_check
        [ "${config_topio_mining_key_pswd}" != "${mining_key_pswd_check}" ] && echo -e "${red}The passwords entered twice are different!${plain}" && echo && continue
        break
    done;
}

function _pre_install_topio_user() {
    # get user that launch topio
    echo -e "${green} Please Input the user which start topio:( Default User is root ):${plain}"
    while true; do
        read -p "(Please Input):" config_topio_user
        [ -z "${config_topio_user}" ] && config_topio_user="root"
        ! $(id "${config_topio_user}" >/dev/null 2>&1 ) && echo -e "[${red}Error${plain}] user ${config_topio_user} not exist" && echo && continue
        break;
    done

    echo
    echo "----------------------------------------------------------------"
    echo "TOPIO User: ${config_topio_user}"
    echo "----------------------------------------------------------------"
    echo
}

function pre_install() {

    echo "pre_install"

    _pre_install_machine_id
    ! _pre_quite_check_package_dir && _pre_install_package_dir
    ! _pre_quite_check_data_dir && _pre_install_data_dir
    ! _pre_quite_check_mining_key && _pre_install_mining_key
    _pre_install_mining_key_pswd
    _pre_install_topio_user
}

# check config file
function check_config() {
    if [ -f ${target_dir}/${bin_name} ]; then
        ${target_dir}/${bin_name} -c ${config_dir}/config.json --check 
        if [ $? -ne 0 ]; then
            echo "${proj_name} install failed, config check error"
            exit 1
        fi
    else
        echo "${proj_name} install failed, please contact @top"
        exit 1
    fi
    
}

# write binary config file
function write_top_auto_upgrader_config() {
    [ ! -d ${config_dir} ] && mkdir ${config_dir}

    cat > ${config_dir}/config.json <<-EOF
{
    "user_config": {
        "minging_keystore_file_dir": "${config_topio_mining_keystore_file}",
        "mining_pub_key": "${config_topio_mining_pub_key}",
        "mining_pswd_enc": "",
        "topio_package_dir": "${config_topio_package_dir}",
        "topio_user": "${config_topio_user}"
    },
    "env_config": {
        "machine_id": "${config_machine_id}"
    },
    "au_config": {

    },
    "temp_config": {
        "temp_pswd": "${config_topio_mining_key_pswd}"
    }
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

function build_or_fetch_top_auto_upgrader() {
    # TODO change from
    # test only
    /bin/cp -rfa ./target/release/${bin_name} ${target_dir}
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
    pre_install

    # 2. echo double check ready
    echo
    echo "Press any key to start install...or Press Ctrl+C to cancel"
    char=$(get_char)

    cd ${cur_dir}

    # 3. install neccessary build tools

    # 4. uninstall old auto upgrader
    do_uninstall_action

    # 5. build or fetch new one
    build_or_fetch_top_auto_upgrader

    # 6. write config
    write_top_auto_upgrader_config

    # 7. use top-auto-upgrader to check config
    check_config

    # 8. install service
    install_top_au_service


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
            echo "Usage: $(basename "$0") [install|uninstall]"
            ;;
    esac

    exit 0
}


#=================================================================#
#                    script begin entry                           #
#=================================================================#

main "$1"