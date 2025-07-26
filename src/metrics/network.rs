use sysinfo::Networks;

pub struct NetworkCollector {
    networks: Networks,
    prev_rx: u64,
    prev_tx: u64,
}

impl NetworkCollector {
    pub fn new() -> Self {
        let networks = Networks::new_with_refreshed_list();
        let (rx, tx) = Self::aggregate(&networks);
        Self {
            networks,
            prev_rx: rx,
            prev_tx: tx,
        }
    }

    fn aggregate(networks: &Networks) -> (u64, u64) {
        let mut rx_total = 0;
        let mut tx_total = 0;

        for (_interface_name, data) in networks {
            rx_total += data.total_received();
            tx_total += data.total_transmitted();
        }

        (rx_total, tx_total)
    }

    pub fn collect_rx(&mut self) -> u64 {
        self.networks.refresh(true);
        let (rx, _) = Self::aggregate(&self.networks);
        let delta = rx.saturating_sub(self.prev_rx);
        self.prev_rx = rx;
        delta
    }

    pub fn collect_tx(&mut self) -> u64 {
        self.networks.refresh(true);
        let (_, tx) = Self::aggregate(&self.networks);
        let delta = tx.saturating_sub(self.prev_tx);
        self.prev_tx = tx;
        delta
    }
}
