// Imports?
pub struct Peer {
    ip: [u8; 4], // currently only support IPv4
    port: u16,
}

impl Peer {
    // Custom error?
    pub fn unmarshal(peers_bin: Vec<u8>) -> Result<Vec<Peer>, Error> {
        // static peerSize instead?
        const PEER_SIZE = 6; // 4 for IP, 2 for port
        let num_peers = peers_bin.length / PEER_SIZE;
        if peers_bin.length % PEER_SIZE != 0 { /* Throw error */ }

        let mut peers = Vec::new();

        // How to iterate through peers_bin with step size peerSize? 

        Ok(peers)
    }
}

// implement toStr or Display or whatever it is
