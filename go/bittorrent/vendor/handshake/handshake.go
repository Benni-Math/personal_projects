package handshake

// A Handshake is a special message that a peer uses to identify itself
type Handshake struct {
    Pstr     string
    InfoHash [20]byte
    PeerID   [20]byte
}

// Serialize the handshake to a buffer
func (h *Handshake) Serialize() []byte {
    buf := make([]byte, len(h.Pstr)+49)
    buf[0] = byte(len(h.Pstr)) //0x13 (19) - length of protocol identifier

    curr := 1
    curr += copy(buf[curr:], h.Pstr) // 'BitTorrent protocol' - identifier
    curr += copy(buf[curr:], make([]byte, 8)) // 8 reserved bytes
    curr += copy(buf[curr:], h.InfoHash[:]) // SHA-1 hashed file identifier
    curr += copy(buf[curr:], h.PeerId[:]) // peer ID which identifies us

    return buf
}

func Read(r io.Reader) (*Handshake, error) {
    // Do Serialize(), but backwards
    // ...
}

