use nvml_wrapper::{Device, Nvml, enum_wrappers::device::EncoderType, error::NvmlError};
use shared::{debug, error, info};

pub fn encoder_check() {
    match Nvml::init() {
        Ok(nvml) => {
            let device_count = nvml.device_count().unwrap();
            debug!("nvml device count: {}", device_count);
            // fixme: on multi-gpu nvidia system will do it twice,
            for index in 0..device_count {
                match nvml.device_by_index(index) {
                    Ok(device) => {
                        debug!("nvml device name: {}", device.name().unwrap());
                        probe_nvenc_encoder_profile(&device, EncoderType::H264, "H264");
                        probe_nvenc_encoder_profile(&device, EncoderType::HEVC, "HEVC");
                        // todo: probe for AV1 when will be available in nvml-wrapper
                    }
                    Err(e) => {
                        error!("Failed to acquire NVML device with error: {}", e)
                    }
                }
            }
        }
        Err(e) => shared::show_e(format!("Can't initialize NVML engine, error: {e}.")),
    }
}

fn probe_nvenc_encoder_profile(device: &Device, encoder_type: EncoderType, profile_name: &str) {
    match device.encoder_capacity(encoder_type) {
        Ok(_) => {
            info!("GPU supports {} profile.", profile_name);
        }
        Err(e) => {
            if matches!(e, NvmlError::NotSupported) {
                shared::show_e(format!(
                    "Your NVIDIA gpu doesn't support {profile_name}. Please make sure CUDA is installed properly. Error: {e}"
                ))
            } else {
                error!("{}", e)
            }
        }
    }
}
