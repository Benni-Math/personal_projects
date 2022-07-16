//go:build exclude
package walkthrough

import (
	"github.com/jackpal/bencode-go"
)

// structure for holding outcome of bencode parsing
type bencodeTorrent struct {
	Announce string      `bencode:"announce"`
	Info     bencodeInfo `bencode:"info"`
}

type bencodeInfo struct {
	Pieces      string `bencode:"pieces"`
	PieceLength int    `bencode:"piece length"`
	Length      int    `bencode:"length"`
	Name        string `bencode:"name"`
}

// Open parses a torrent file
func Open(r io.Reader) (*bencodeTorrent, error) {
	bto := bencodeTorrent{}
	err := bencode.Unmarshal(r, &bto)
	if err != nil {
		return nil, err
	}
	return &bto, nil
}

// Flatter (public) struct with more segmented info
// i.e. we break up the SHA-1 binary hash info
type TorrentFile struct {
	Announce    string
	InfoHash    [20]byte
	PieceHashes [][20]byte
	PieceLength int
	Length      int
	Name        string
}

// Transforming from parsed to exposed struct
func (bto bencodeTorrent) toTorrentFile() (TorrentFile, error) {

}

// Announcing our presence as a peer via GET
// Returns a bencoded response with an interval and a binary blob of peers
func (t *TorrentFile) buildTrackerURL(peerID [20]byte, port uint16) (string, error) {
	// The announce URL is in the .torrent file
	base, err := url.Parse(t.Announce)
	if err != nil {
		return "", err
	}

	params := url.Values{
		// Identifies the file we're trying to download
		"info_hash": []string{string(t.InfoHash[:])},
		// A (random) 20 byte name to identify ourselves to trackers and peers
		"peer_id":    []string{string(peerID[:])},
		"port":       []string{strconv.Itoa(int(Port))},
		"uploaded":   []string{"0"},
		"downloaded": []string{"0"},
		"compact":    []string{"1"},
		"left":       []string{strconv.Itoa(t.Length)},
	}
	base.RawQuery = params.Encode()

	return base.String(), nil
}

// The 'peer blob' is made out of groups of six bytes
// The first four are the IPv4 address, and the last two are
// a big-endian uin16
type Peer struct {
	IP   net.IP
	Port unint16
}

// Unmarshal parses peer IP addresses and ports from a buffer
func Unmarshal(peersBin []byte) ([]Peer, error) {
	const peerSize = 6 // 4 for IP, 2 for port
	numPeers := len(peersBin) / peerSize

	if len(peersBin)%peerSize != 0 {
		err := fmt.Errorf("Received malformed peers")
		return nil, err
	}

	peers := make([]Peer, numPeers)
	for i := 0; i < numPeers; i++ {
		offset := i * peerSize
		peers[i].IP = net.IP(peersBin[offset : offest+4])
		peers[i].Port = binary.BigEndian.Uint16(peersBin[offset+4 : offset+6])
	}

	return peers, nil
}

// Downloading from peers
//  1. Start a TCP connection with the peer
//  2. Complete a two-way BitTorrent handshake
//  3. Exchange messages to download pieces - "I'd like piece #231 please"
