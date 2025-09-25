NaNVR can be built only on Linux.

# Common Prerequisites

Preferred IDE (optional): Visual Studio Code with `rust-analyzer` and `clangd` extension

You need to install [rustup](https://www.rust-lang.org/tools/install).

To clone the repository use `git clone --recurse-submodules https://github.com/nanvr/NaNVR.git`.
If you previously cloned the repo without submodules, simply run `git submodule update --init --checkout --recursive` in it.

Adjust path in `.clangd` to absolute path for clangd to have proper intellisense over openvr files

# Streamer Building

First you need to gather some additional resources in preparation for the build.  

Install these additional packages:

* **Arch**
  
  Note: At time of writing Arch gcc is too new to be compatible with NVIDIA's nvcc. This means there is no neat way to compile an nvidia compatible build. For now, you can set this environment variable for build:
  `export NVCC_APPEND_FLAGS+='-std=c++14'`

  ```bash
  sudo pacman -S clang curl nasm pkgconf yasm vulkan-headers libva-mesa-driver unzip ffmpeg libpipewire
  ```

* **Gentoo** (fixme: outdated, ffmpeg shouldn't be required anymore)
  
  * `media-video/ffmpeg >= 4.4 [encode libdrm vulkan vaapi]`
  * `sys-libs/libunwind`
  * `dev-lang/rust >= 1.72`
  * `media-video/pipewire [jacksdk]`

* **Debian 12 / Ubuntu 20.04 / Pop!\_OS 20.04**
  
  ```bash
  sudo apt install pulseaudio-utils build-essential pkg-config libclang-dev libssl-dev libasound2-dev libjack-dev libgtk-3-dev libvulkan-dev libunwind-dev gcc yasm nasm curl libx264-dev libx265-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libdrm-dev libva-dev libvulkan-dev vulkan-headers libpipewire-0.3-dev libspa-0.3-dev git
  ```

  * Note: Libpipewire/libspa must be at least 0.3.49 version - make sure to use upstream pipewire <https://github.com/pipewire-debian/pipewire-debian>

* **Fedora**
  
  ```bash
  sudo dnf groupinstall 'Development Tools' | For c++ and build tools
  sudo dnf install nasm yasm libdrm-devel vulkan-headers pipewire-jack-audio-connection-kit-devel atk-devel gdk-pixbuf2-devel cairo-devel rust-gdk0.15-devel x264-devel vulkan-devel libunwind-devel clang openssl-devel alsa-lib-devel libva-devel pipewire-devel git
  ```
  
  If you are using Nvidia, see [Fedora cuda installation](Building-From-Source#fedora-cuda-installation)

Move to the root directory of the project, then run this command (paying attention to the bullet points below):

```bash
cargo xtask build-server-deps
```

* Use the `--enable-nvenc` flag if you have a Nvidia GPU.

Next up is the proper build of the streamer. Run the following:

```bash
cargo xtask build-streamer --release
```

You can find the resulting package in `build/nanvr_streamer`

If you want to edit and rebuild the code, you can skip the `build-server-deps` command and run only the `build-streamer` command.

## Fedora CUDA installation

If you are here for CUDA installation on Fedora you're at the right place! Else continue down to [Android App Building](Building-From-Source#android-app-building)

### 1. Install Nvidia drivers and Fedora CUDA driver

```bash
sudo dnf update -y
```

(Reboot if you have a new kernel)

```bash
sudo dnf install akmod-nvidia
sudo dnf install xorg-x11-drv-nvidia-cuda
```

Wait until ```modinfo -F version nvidia``` doesn't report ```"ERROR: Module nvidia not found"``` anymore

### 2. Install Nvidia's CUDA

In the previous step, we installed Fedora's CUDA that doesn't work with NaNVR, installing Nvidia's CUDA works and creates directories instead

```bash
sudo dnf config-manager --add-repo https://developer.download.nvidia.com/compute/cuda/repos/fedora37/x86_64/cuda-fedora37.repo
```

Change the Fedora version if you are on a different version. You should check if your version is supported by inspecting the repo

```bash
sudo dnf clean all
sudo dnf module disable nvidia-driver
sudo dnf -y install cuda
export PATH=/usr/local/cuda-12.3/bin${PATH:+:${PATH}}
```

If your cuda version is different, change it to the version that is installed. You can check installed versions by doing ```ls /usr/local/ | grep "cuda"``` in your terminal

#### Note about Nvidia's CUDA

* Disabling the nvidia-driver doesn't disable Nvidia drivers but prevents nvidia dkms from installing over the akmod driver

### 3. Install gcc11 install with homebrew

Becuase cuda cannot be ran without a gcc version lower than or equal to gcc12, you will need to install a gcc version on homebrew. The fedora gcc11 package got removed so this is the only way sadly
To install homebrew, run this command:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

Then install gcc11

```bash
brew install gcc@11
```

#### Notes on installing gcc11 with homebrew

* If brew is not found in your path, run the following separately to add brew to your path:
  
  ```bash
  test -d ~/.linuxbrew && eval "$(~/.linuxbrew/bin/brew shellenv)" 
  test -d /home/linuxbrew/.linuxbrew && eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"
  echo "eval \"\$($(brew --prefix)/bin/brew shellenv)\"" >> ~/.bashrc
  ```

### 4. Modify dependencies.rs to use correct cuda path and gcc version

Because CURA installs as a symlink by default, we need to change the dependencies.rs to use the directory
From the NaNVR directory edit the ./nanvr/xtask/src/dependencies.rs, and change two lines:

* Line 159, change ```cuda``` -> ```cuda-12.3``` (or whatever version you have)
* Line 179, replace that line with ```--nvccflags=\"-ccbin /home/linuxbrew/.linuxbrew/bin/g++-11 -gencode arch=compute_52,code=sm_52 -O2\"``` (Change homebrew path if needed, default is used)

You should be good to go! Refer to [Streamer Building](Building-From-Source#streamer-building) for the commands to build application

# Android App Building

## 1. Installing necessary packages

For the app you need install:

* [Android Studio](https://developer.android.com/studio) or the [sdkmanager](https://developer.android.com/studio/command-line/sdkmanager)
* Android SDK Platform-Tools 29 (Android 10)
* Latest Android NDK (currently v25.1.8937393)

Specific package names for the android tools can differ from distro to distro, see up on the wiki for more information:

* Gentoo:
  * <https://wiki.gentoo.org/wiki/Android>
* Arch:
  * <https://wiki.archlinux.org/title/Android>
* Debian:
  * <https://wiki.debian.org/AndroidStudio>
* Ubuntu:
  * <https://help.ubuntu.com/community/AndroidSDK>
* Pop!\_OS:
  * N/A

The three mentioned developer applications can be installed from upstream; although the packages and setup responsible for the required tools can differ between distros, being:

* **Arch**
  * Packages can vary, read up on the Arch Wiki's [Android](https://wiki.archlinux.org/title/Android) page.
* **Gentoo**
  * `dev-util/android-studio`
  * `dev-util/android-sdk-update-manager`
  * `dev-util/android-ndk >= 25.1`

For Debian, it requires to have the `non-free` repository to be enabled:

* **Debian 12 / Ubuntu 22.10 / Pop!\_OS 22.10**
  
  ```bash
  sudo apt install android-sdk-platform-tools-common sdkmanager google-android-ndk-r26b-installer
  ```
  
## 2. Setting environment variables

Correct directories for the environment variables can greatly differ depending on the type of install. See the wiki page of your distro for more information:

* Arch:
  * <https://wiki.archlinux.org/title/Android#App_development>
* Gentoo:
  * <https://wiki.gentoo.org/wiki/Android>
* Ubuntu:
  * <https://help.ubuntu.com/community/AndroidSDK#Post-Installation_Configuration>

Distro wikis that weren't listed above does not mention of environment variables, although generally they would be as:

* `JAVA_HOME`:
  * `/usr/lib/jvm/default-java/bin`
* `ANDROID_HOME`:
  * Arch: `~/Android/Sdk`
  * Gentoo: `~/Android`
  * Debian / Ubuntu / Pop!\_OS: `~/AndroidSDK`
* `ANDROID_NDK_HOME`:
  * Arch: `/opt/android-sdk/ndk`
  * Linux: `/usr/lib/android-sdk/ndk`

## 3. Building

First you need to gather some additional resources in preparation for the build.  
Move to the root directory of the project, then run this command:

```bash
cargo xtask prepare-deps --platform android
```

Before building the app, Android has to have us to agree to the licenses otherwise building the app will halt and fail. To accept the agreements, follow the instructions for your corresponding OS:

  ```bash
  cd ~/AndroidSDK
  sdkmanager --licenses
  ```

Next up is the proper build of the app. Run the following:

```bash
cargo xtask build-client --release
```

The built APK will be in `build/nanvr_client_android`. You can then use adb or SideQuest to install it to your headset.

To build and run:

```bash
cd nanvr/client_openxr
cargo apk run
```

You need the headset to be connected via USB and with the screen on to successfully launch the debugger and logcat.
