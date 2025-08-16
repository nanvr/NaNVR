<!-- <p align="center"> <img width="500" src="resources/ALVR-Grey.svg"/> </p> -->

# NaNVR

## !Project is currently under heavy development, expect a lot of breaking changes!

Stream VR games from your PC to your headset via Wi-Fi.
This is a fork of [ALVR](https://github.com/alvr-org/ALVR).

### Direct download to the latest version:
### [Linux Launcher](https://github.com/nanvr/NANVR/releases/latest/download/launcher_linux.tar.gz)

## Compatibility

|          VR Headset          |                                        Support                                         |
| :--------------------------: | :------------------------------------------------------------------------------------: |
|       Apple Vision Pro       |             :x: ([supported under alvr](https://github.com/alvr-org/alvr-visionos))    |
|      Quest 1/2/3/3S/Pro      |                                   :heavy_check_mark:                                   |
|     Pico Neo 3/4/4 Ultra     |                                   :heavy_check_mark:                                   |
|    Play For Dream YVR 1/2/MR |                                   :heavy_check_mark:                                   |
| Vive Focus 3/Vision/XR Elite |                                   :heavy_check_mark:                                   |
|           Lynx R1            |                                   :heavy_check_mark:                                   |
|     PhoneVR (smartphone)     |     :heavy_check_mark: ** ([repo](https://github.com/PhoneVR-Developers/PhoneVR))      |
|        Android/Monado        |                                      :warning: **                                      |
|          Oculus Go           |                 :x: ([old repo](https://github.com/polygraphene/ALVR))                 |

\* : ALVR for Quest 1 not available through the Meta store.  
\** : Only works on some smartphones, not enough testing.  

|     PC OS      |                                    Support                                    |
| :------------: | :---------------------------------------------------------------------------: |
|     Linux      |                             :heavy_check_mark:                                |


### Requirements

-   A supported standalone VR headset (see compatibility table above)

-   SteamVR

-   High-end gaming PC
    -   NVIDIA GPU that supports NVENC (1000 GTX Series or higher) (or with an AMD GPU that supports AMF VCE) with the latest driver.
    -   Laptops with an onboard (Intel HD, AMD iGPU) and an additional dedicated GPU.

-   802.11ac 5Ghz wireless or ethernet wired connection
    -   It is recommended to use 802.11ac 5Ghz for the headset and ethernet for PC
    -   You need to connect both the PC and the headset to same router (or use a routed connection as described [here](https://github.com/nanvr/NANVR/wiki/ALVR-v14-and-Above))

## Install

Follow the installation guide [here](https://github.com/nanvr/NANVR/wiki/Installation-guide).

## Troubleshooting

-   Please check the [Troubleshooting](https://github.com/nanvr/NANVR/wiki/Troubleshooting) page, and also [Linux Troubleshooting](https://github.com/nanvr/NANVR/wiki/Linux-Troubleshooting) if applicable.
-   Configuration recommendations and information may be found [here](https://github.com/nanvr/NANVR/wiki/Information-and-Recommendations)

## Uninstall (TODO)

## Build from source

You can follow the guide [here](https://github.com/nanvr/NANVR/wiki/Building-From-Source).

## License

NaNVR is licensed under the [MIT License](LICENSE). (TODO: possibly will be GPL only due to ffmpeg, x264 usage, GPL only build in any case)

## Privacy policy

NaNVR apps do not directly collect any kind of data.
