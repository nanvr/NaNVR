use mdns_sd::{ServiceDaemon, ServiceInfo};
use shared::{
    NANVR_LOW_NAME,
    anyhow::{Result, bail},
};

pub struct AnnouncerSocket {
    hostname: String,
    daemon: ServiceDaemon,
}

impl AnnouncerSocket {
    pub fn new(hostname: &str) -> Result<Self> {
        let daemon = ServiceDaemon::new()?;

        Ok(Self {
            daemon,
            hostname: hostname.to_owned(),
        })
    }

    pub fn announce(&self) -> Result<()> {
        let local_ip = system_info::local_ip();
        if local_ip.is_unspecified() {
            bail!("IP is unspecified");
        }

        self.daemon.register(ServiceInfo::new(
            net_sockets::MDNS_SERVICE_TYPE,
            &format!("{NANVR_LOW_NAME}{}", rand::random::<u16>()),
            &self.hostname,
            local_ip,
            5353,
            &[(
                net_sockets::MDNS_PROTOCOL_KEY,
                shared::protocol_id().as_str(),
            )][..],
        )?)?;

        Ok(())
    }
}
