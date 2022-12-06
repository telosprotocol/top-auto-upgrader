#!/bin/bash
# description: Daemon Script of Auto Upgrade Service for TOP-Chain.

### BEGIN INIT INFO
# Provides:          top-au
# Required-Start:    $network $syslog
# Required-Stop:     $network
# Default-Start:     2 3 4 5
# Default-Stop:      0 1 6
# Short-Description: Start daemon of Auto Upgrade Service for TOP-Chain.
# Description:       Control Auto Upgrade Service for TOP-Chain.
### END INIT INFO

# Author: Charles

NAME="TOP Auto Upgrader"
DAEMON=/usr/bin/top-auto-upgrader
CONF=/etc/top-au/config.json

command_start="${DAEMON} -d -c ${CONF}"

PID=0
RETVAL=0

check_running() {
    PID=$(ps -ef | grep -v grep | grep -i "${DAEMON}" | awk '{print $2}')
    if [ -n "${PID}" ]; then
        return 0
    else
        return 1
    fi
}

do_start() {
    check_running
    if [ $? -eq 0 ]; then
        echo "${NAME} (pid ${PID}) is already running..."
        exit 0
    else
        ${command_start}
        RETVAL=$?
        if [ "${RETVAL}" -eq 0 ]; then
            echo "Starting ${NAME} success"
        else
            echo "Starting ${NAME} failed"
        fi
    fi
}

do_stop() {
    check_running
    if [ $? -eq 0 ]; then
        kill "${PID}"
        RETVAL=$?
        if [ "${RETVAL}" -eq 0 ]; then
            echo "Stopping ${NAME} success"
        else
            echo "Stopping ${NAME} failed"
        fi
    else
        echo "${NAME} is stopped"
        RETVAL=1
    fi
}

do_status() {
    check_running
    if [ $? -eq 0 ]; then
        echo "${NAME} (pid ${PID}) is running..."
    else
        echo "${NAME} is stopped"
        RETVAL=1
    fi
}

do_restart() {
    do_stop
    sleep 0.5
    do_start
}

case "${1}" in
    start|stop|restart|status)
        do_"${1}"
        ;;
    *)
        echo "Usage: ${0} { start | stop | restart | status }"
        RETVAL=1
        ;;
esac

exit ${RETVAL}
