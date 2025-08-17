#!/bin/sh -e

export ADB_VENDOR_KEYS=~/.android/adbkey.pub
flatpak override --user --filesystem=~/.android com.valvesoftware.Steam.Utility.nanvr
flatpak run --env=ADB_VENDOR_KEYS=$ADB_VENDOR_KEYS --env=QT_QPA_PLATFORM=xcb --command=nanvr_launcher com.valvesoftware.Steam