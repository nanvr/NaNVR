## Launcher

Launcher will allow you to manage old, current and new installations of NaNVR streamer and allow to automatically install and upgrade to specific NaNVR app version on headset

### Installation

* Download `launcher_linux.tar.gz` from the release [download page](https://github.com/nanvr/NaNVR/releases/latest) and extract into path without root rights.
* Run `launcher_linux/NaNVR Launcher`
* Press `Add version` button
* For default installation keep channel and version as is and press `Install`
* Wait until it finishes downloading, installing (depends on your connection)
* To install NaNVR app on headset, use button `Install APK`
* In the list, to open streamer app (PC) press `Launch`. You will be greeted with a setup wizard. Follow the setup to set the firewall rules and other settings.

### Usage

* Before launching SteamVR through NaNVR, please install it. First time launch will result in steamvr being blank and nanvr will not work - close it and start again. It will have registered driver and should work.
* Launch NaNVR app on your headset. While the headset screen is on, click `Trust` next to the device entry (in the NaNVR streamer app on PC, in the `Devices` tab) to start streaming.
* You can change settings on the PC in the `Settings` tab. Most of the settings require to restart SteamVR to be applied. Use the apposite button on the bottom right corner.

For any problem visit the [Troubleshooting page](Troubleshooting).

## Advanced installation

### Manually installing NaNVR streamer

There is also a portable version for the PC that requires more manual steps to make it work.

* Download `nanvr_streamer_linux.tar.gz` from the release [download page](https://github.com/nanvr/NaNVR/releases/latest), extract it.
* Run `bin/dashboard`

#### Nightly

If you want to get new features early or you want to help with testing you can install a nightly version.

Download the latest nightly streamer [here](https://github.com/nanvr/NaNVR-nightly/releases/latest).

Since nightly releases can be unstable, always use matching versions for PC and headset. They are updated once a day.

### Flatpak

For Flatpak users, refer to the instructions [here](Installing-NaNVR-and-using-SteamVR-on-Linux-through-Flatpak)

## Advanced usage

### Use together with third-party drivers

By default NaNVR disables other SteamVR drivers before starting. Among these drivers there is [SlimeVR](https://slimevr.dev/) for body tracking. NaNVR disables these drivers to maximize compatibility with every PC setup. You can disable this behavior by manually registering the NaNVR driver. Go to the `installation` tab and click on `Register NaNVR driver`. The next time you launch NaNVR you will be able to use the other drivers concurrently.

### Launch together with SteamVR

You can skip the Dashboard and open NaNVR automatically together with SteamVR.

**Note:** You can only do that while SteamVR is not already running. Otherwise driver might be unregistered on shutdown.

Open Dashboard, go to the `Installation` tab and click on `Register NaNVR driver`.

### Connect headset to PC via a USB Cable

Check out the guide [here](NaNVR-over-USB).
