#!/bin/bash
DST_START=2024-03-31
DST_END=2024-10-27
function summer()
{
    ILLEGAL_SUMMER_BEFORE="${DST_START}T02:30+0100"
    ILLEGAL_SUMMER_AFTER="${DST_START}T02:30+0200"
    SUMMER_BEFORE="${DST_START}T01:59+0100"
    SUMMER_AFTER="${DST_START}T03:01+0200"

    echo "Illegal timestamps, there is no 02:30 on this day (neither CET nor CEST)"
    date -d $ILLEGAL_SUMMER_BEFORE
    date -d $ILLEGAL_SUMMER_AFTER

    echo "Times right before and after change in summer"
    date -d $SUMMER_BEFORE
    date -d $SUMMER_AFTER
}

function winter()
{
    WINTER_BEFORE="${DST_END}T02:30+0200"
    WINTER_AFTER="${DST_END}T02:30+0100"

    echo "02:30 before return to standard time (CET): $WINTER_BEFORE"
    date -d $WINTER_BEFORE
    echo "02:30 after return to standard time (CET): $WINTER_AFTER"
    date -d $WINTER_AFTER
}

function duration()
{
    local startTime=$1
    local endTime=$2
    local START=$(date -d $startTime +%s)
    local END=$(date -d $endTime +%s)
    local DURATION=$((END-START))
    local HOURS=$(($DURATION / 3600))
    local MINUTES=$((($DURATION % 3600) / 60))
    local SECONDS=$(($DURATION % 60))
    printf "%02d:%02d:%02d\n" $HOURS $MINUTES $SECONDS
}

{
    # summer
    # winter

    TIME_START="T00:30:00"
    TIME_END="T04:30:00"
    echo "Duration on the day of change to standard time"
    duration "${DST_END}${TIME_START}+0200" "${DST_END}${TIME_END}+0100"

    ST_DATE=2024-12-15
    echo "Duration on a day during standard time"
    duration "${ST_DATE}${TIME_START}+0100" "${ST_DATE}${TIME_END}+0100"

    DST_DATE=2024-07-15
    echo "Duration on a day during daylight saving time"
    duration "${DST_DATE}${TIME_START}+0200" "${DST_DATE}${TIME_END}+0200"

    echo "Duration on the day of change to daylight saving time"
    duration "${DST_START}${TIME_START}+0100" "${DST_START}${TIME_END}+0200"
}