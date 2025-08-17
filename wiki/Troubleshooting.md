Troubleshooting
===

### Hyprland/Sway/Wlroots Qt fix (todo: move to dashboard)

If you're on hyprland, sway, or other wlroots-based wayland compositor, you might have to prepend `QT_QPA_PLATFORM=xcb` before commandline, which results in full commandline for steamvr being something like this:
`QT_QPA_PLATFORM=xcb ~/.local/share/Steam/steamapps/common/SteamVR/bin/vrmonitor.sh %command%`.

Related issue:
[[BUG] No SteamVR UI on wlroots-based wayland compositors (sway, hyprland, ...) with workaround](https://github.com/ValveSoftware/SteamVR-for-Linux/issues/637).

## Nvidia driver version requirements

Application requires at least driver version 535 and CUDA version 12.1. If this is not the case SteamVR or the encoder might not work.

### Fix

Install at least the required versions of the driver and ensure you have CUDA installed with at least version 12.1.

If an error saying CUDA was not detected persists, try using the latest nanvr nightly release.

## Using only integrated graphics

Beware that using **only** integrated graphics for running application is highly inadvisable as in most cases it will lead to very poor performance (even on more powerful devices like Steam Deck, it's still very slow).
Don't expect things to work perfectly in this case too, as some older integrated graphics simply might not have the best vulkan support and might fail to work at all. 

## Hybrid graphics advices (todo: check if needs more info)

### General advise

If you have PC and can disable your integrated gpu from BIOS/UEFI, it's highly advised to do so to avoid multiple problems of handling hybrid graphics.
If you're on laptop and it doesn't allow disabling integrated graphics (in most cases) you have to resort to methods bellow.

### Amd/Intel integrated gpu + Amd/Intel discrete gpu

Put `DRI_PRIME=1 ~/.local/share/Steam/steamapps/common/SteamVR/bin/vrmonitor.sh %command%` (adjust vrmonitor path to your distro) into SteamVR's commandline options and in those of all VR games you intend to play with NaNVR.

### Amd/Intel integrated gpu + Nvidia discrete gpu

Put `__NV_PRIME_RENDER_OFFLOAD=1 __VK_LAYER_NV_optimus=NVIDIA_only __GLX_VENDOR_LIBRARY_NAME=nvidia ~/.local/share/Steam/steamapps/common/SteamVR/bin/vrmonitor.sh %command%` (adjust vrmonitor path to your distro) into SteamVR's commandline options and in those of all VR games you intend to play.

If this results in errors such as `error in encoder thread: Failed to initialize vulkan frame context: Invalid argument`, then try this instead:

`__NV_PRIME_RENDER_OFFLOAD=1 __VK_LAYER_NV_optimus=NVIDIA_only __GLX_VENDOR_LIBRARY_NAME=nvidia VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/nvidia_icd.json  ~/.local/share/Steam/steamapps/common/SteamVR/bin/vrmonitor.sh %command%`

- Again, adjust vrmonitor path to your distro
- Go to `/usr/share/vulkan/icd.d` and make sure `nvidia_icd.json` exists. It may also be under the name `nvidia_icd.x86_64.json`, in which case you should adjust `VK_ICD_FILENAMES` accordingly.

### SteamVR Dashboard not rendering in VR on Nvidia discrete GPU
If you encounter issues with the SteamVR dashboard not rendering in VR you may need to run the entire steam client itself via PRIME render offload. First close the steam client completey if you have it open already, you can do so by clicking the Steam dropdown in the top left and choosing exit. Then from a terminal run: `__NV_PRIME_RENDER_OFFLOAD=1 __GLX_VENDOR_LIBRARY_NAME=nvidia steam-runtime`

## Wayland

When using old Gnome (< 47 version) under Wayland you might need to put `WAYLAND_DISPLAY='' ~/.local/share/Steam/steamapps/common/SteamVR/bin/vrmonitor.sh %command%` (adjust vrmonitor path to your distro) into the SteamVR commandline options to force XWayland on SteamVR. This fixes issue with drm leasing not being available.

## The view shakes

SlimeVR related, might be fixed in future updates of NaNVR

### Fix

Start the SlimeVR Server only after you connected and got an image to headset at least once.

## No audio or microphone (todo: add check on nanvr side to notify user)

Even though audio or microphone are enabled in presets, still can't hear audio or no one can hear me

### Fix

Make sure you select `NaNVR Audio` and `NaNVR Microphone` in device list as default **after** connecting headset. As soon as headset disconnected, devices will be removed. If you set it as default, they will be automatically chosen whenever they show up and you don't need to do it manually ever again.
If you don't appear to have audio devices, or have pipewire errors in logs, check if you have `pipewire` installed and it's at least version `0.3.49` by using command `pipewire --version`
For older (<=22.04 or debian <=11) ubuntu or debian based distributions you can check [pipewire-upstream](https://github.com/pipewire-debian/pipewire-debian) page for installing newer pipewire version

## OVR Advanced Settings

Disable the OVR Advanced Settings driver and don't use it with NaNVR.
It's incompatible and will produce ladder-like latency graph with very bad shifting vision.


## Bindings not working/high cpu usage due to bindings ui

Steamvr can't properly update bindings, open menus, and possibly eats too much cpu.

This issue is caused by SteamVR's webserver spamming requests that stall the chromium ui and causes it to use a lot of cpu.

### Fix

Apply the following patch: `https://github.com/alvr-org/ALVR-Distrobox-Linux-Guide/blob/main/patch_bindings_spam.sh`
Assuming default path for Arch, Fedora - one-liner: `curl -s https://raw.githubusercontent.com/alvr-org/ALVR-Distrobox-Linux-Guide/main/patch_bindings_spam.sh | sh -s ~/.steam/steam/steamapps/common/SteamVR`
