mod nvenc;
mod vaapi;
use std::path::Path;

use shared::{debug, error, info, warn};

#[derive(PartialEq)]
enum DeviceInfo {
    Nvidia,
    Amd { device_type: wgpu::DeviceType },
    Intel { device_type: wgpu::DeviceType },
    Unknown,
}

pub fn audio_check() {
    // No check for result, just show errors in logs
    let _ = sound::linux::try_load_pipewire();
}

pub fn hardware_checks() {
    let wgpu_adapters = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        ..Default::default()
    })
    .enumerate_adapters(wgpu::Backends::VULKAN);
    let device_infos = wgpu_adapters
        .iter()
        .filter(|adapter| {
            adapter.get_info().device_type == wgpu::DeviceType::DiscreteGpu
                || adapter.get_info().device_type == wgpu::DeviceType::IntegratedGpu
        })
        .map(|adapter| {
            let vendor = match adapter.get_info().vendor {
                0x10de => DeviceInfo::Nvidia,
                0x1002 => DeviceInfo::Amd {
                    device_type: adapter.get_info().device_type,
                },
                0x8086 => DeviceInfo::Intel {
                    device_type: adapter.get_info().device_type,
                },
                _ => DeviceInfo::Unknown,
            };

            (adapter, vendor)
        })
        .collect::<Vec<_>>();

    gpu_checks(&device_infos);
    encoder_checks(&device_infos);
}

fn gpu_checks(device_infos: &[(&wgpu::Adapter, DeviceInfo)]) {
    let have_intel_igpu = device_infos.iter().any(|gpu| {
        gpu.1
            == DeviceInfo::Intel {
                device_type: wgpu::DeviceType::IntegratedGpu,
            }
    });
    debug!("have_intel_igpu: {}", have_intel_igpu);
    let have_amd_igpu = device_infos.iter().any(|gpu| {
        gpu.1
            == DeviceInfo::Amd {
                device_type: wgpu::DeviceType::IntegratedGpu,
            }
    });
    debug!("have_amd_igpu: {}", have_amd_igpu);

    let have_igpu = have_intel_igpu || have_amd_igpu;
    debug!("have_igpu: {}", have_igpu);

    let have_nvidia_dgpu = device_infos.iter().any(|gpu| gpu.1 == DeviceInfo::Nvidia);
    debug!("have_nvidia_dgpu: {}", have_nvidia_dgpu);

    let have_amd_dgpu = device_infos.iter().any(|gpu| {
        gpu.1
            == DeviceInfo::Amd {
                device_type: wgpu::DeviceType::DiscreteGpu,
            }
    });
    debug!("have_amd_dgpu: {}", have_amd_dgpu);

    if have_amd_igpu || have_amd_dgpu {
        let is_any_amd_driver_invalid = device_infos.iter().any(|gpu| {
            info!("Driver name: {}", gpu.0.get_info().driver);
            match gpu.0.get_info().driver.as_str() {
                "AMD proprietary driver" | "AMD open-source driver" => true, // AMDGPU-Pro | AMDVLK
                _ => false,
            }
        });
        if is_any_amd_driver_invalid {
            error!(
                "Amdvlk or amdgpu-pro vulkan drivers detected, SteamVR may not function properly. \
            Please remove them or make them unavailable for SteamVR and games you're trying to launch.\n\
            For more detailed info visit the wiki: \
            https://github.com/alvr-org/ALVR/wiki/Linux-Troubleshooting#artifacting-no-steamvr-overlay-or-graphical-glitches-in-streaming-view"
            )
        }
    }

    let have_intel_dgpu = device_infos.iter().any(|gpu| {
        gpu.1
            == DeviceInfo::Intel {
                device_type: wgpu::DeviceType::DiscreteGpu,
            }
    });
    debug!("have_intel_dgpu: {}", have_intel_dgpu);

    let steamvr_root_dir = match server_io::steamvr_root_dir() {
        Ok(dir) => dir,
        Err(e) => {
            error!(
                "Couldn't detect openvr or steamvr files. \
            Please make sure you have installed and ran SteamVR at least once. \
            Or if you're using Flatpak Steam, make sure to use ALVR Dashboard from Flatpak ALVR. {e}"
            );
            return;
        }
    };

    let vrmonitor_path_string = steamvr_root_dir
        .join("bin")
        .join("vrmonitor.sh")
        .into_os_string()
        .into_string()
        .unwrap();
    debug!("vrmonitor_path: {}", vrmonitor_path_string);

    let steamvr_opts = "For functioning VR you need to put the following line into SteamVR's launch options and restart it:";
    let game_opts = "And this similar line to the launch options of ALL games that you're trying to launch from steam:";

    let mut vrmonitor_path_written = false;
    if have_igpu {
        if have_nvidia_dgpu {
            let base_path = "/usr/share/vulkan/icd.d/nvidia_icd";
            let nvidia_vk_override_path = if Path::new(&format!("{base_path}.json")).exists() {
                format!("VK_DRIVER_FILES={base_path}.json")
            } else if Path::new(&format!("{base_path}.x86_64.json")).exists() {
                format!("VK_DRIVER_FILES={base_path}.x86_64.json")
            } else {
                "__VK_LAYER_NV_optimus=NVIDIA_only".to_string()
            };
            let nv_options = format!(
                "__GLX_VENDOR_LIBRARY_NAME=nvidia __NV_PRIME_RENDER_OFFLOAD=1 {nvidia_vk_override_path}"
            );

            warn!("{steamvr_opts}\n{nv_options} {vrmonitor_path_string} %command%");
            warn!("{game_opts}\n{nv_options} %command%");

            vrmonitor_path_written = true;
        } else if have_intel_dgpu || have_amd_dgpu {
            warn!("{steamvr_opts}\nDRI_PRIME=1 {vrmonitor_path_string} %command%");
            warn!("{game_opts}\nDRI_PRIME=1 %command%");
            vrmonitor_path_written = true;
        } else {
            warn!(
                "Beware, using just integrated graphics might lead to very poor performance in SteamVR and VR games."
            );
            warn!(
                "For more information, please refer to the wiki: https://github.com/alvr-org/ALVR/wiki/Linux-Troubleshooting"
            )
        }
    }
    if !vrmonitor_path_written {
        warn!(
            "Make sure you have put the following line in your SteamVR launch options and restart it:\n\
            {vrmonitor_path_string} %command%"
        )
    }
}

fn encoder_checks(device_infos: &[(&wgpu::Adapter, DeviceInfo)]) {
    for device_info in device_infos {
        match device_info.1 {
            DeviceInfo::Nvidia => nvenc::encoder_check(),
            DeviceInfo::Amd { device_type: _ } | DeviceInfo::Intel { device_type: _ } => {
                vaapi::encoder_check()
            }
            _ => shared::show_e(
                "Couldn't determine gpu for hardware encoding. \
            You will likely fallback to software encoding.",
            ),
        }
    }
}
