use flume::TryRecvError;
use mdns_sd::{Receiver, ServiceDaemon, ServiceEvent};
use shared::{
    ToAny,
    anyhow::{Result, bail},
    warn,
};
use std::{collections::HashMap, net::IpAddr};

pub struct WelcomeSocket {
    mdns_receiver: Receiver<ServiceEvent>,
}

impl WelcomeSocket {
    pub fn new() -> Result<Self> {
        let mdns_receiver = ServiceDaemon::new()?.browse(net_sockets::MDNS_SERVICE_TYPE)?;

        Ok(Self { mdns_receiver })
    }

    // Returns: client IP, client hostname
    pub fn recv_all(&self) -> Result<HashMap<String, IpAddr>> {
        let mut clients = HashMap::new();

        loop {
            match self.mdns_receiver.try_recv() {
                Ok(event) => {
                    if let ServiceEvent::ServiceResolved(info) = event {
                        let hostname = info
                            .get_property_val_str(net_sockets::MDNS_DEVICE_ID_KEY)
                            .unwrap_or_else(|| info.get_hostname());
                        let address = *info.get_addresses().iter().next().to_any()?;

                        let client_protocol = info
                            .get_property_val_str(net_sockets::MDNS_PROTOCOL_KEY)
                            .to_any()?;
                        let server_protocol = shared::protocol_id();

                        if client_protocol != server_protocol {
                            let protocols = format!(
                                "Protocols: server={server_protocol}, client={client_protocol}"
                            );
                            warn!("Found incompatible client {hostname}! {protocols}");
                        }

                        clients.insert(hostname.into(), address);
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(e) => bail!(e),
            }
        }

        Ok(clients)
    }
}
