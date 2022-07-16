pub struct Handshake {
    pstr: &'static str,
    info_hash: [u8; 20], // not sure if this is the right syntax
    peer_id: [u8; 20],
}

impl Handshake {
    pub fn new(info_hash: [u8; 20], peer_id: [u8; 20]) -> Self {
        Handshake {
            // How do I make this static
            pstr: "BitTorrent protocol",
            info_hash: info_hash,
            peer_id: peer_id,
        }
    }

    // Do I want to take ownership here?
    // l_pstr = pstr.length
    pub fn serialize(self) -> [u8; 49+l_pstr] {
        let mut buf = [0u8; 49+l_pstr]
        buf[0] = l_pstr;
        // <pstr><[0u8; 8]><info_hash><peer_id>
        // Find a better iterator/slice method of doing this
        buf
    } 

    pub fn read(/* io.Reader */) -> Result<Self> {
        
    }
}
