pub fn audio_check() {
    // No check for result, just show errors in logs
    let _ = sound::linux::try_load_pipewire();
}
