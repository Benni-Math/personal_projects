package torrentfile

import (
	"net/http"
    "net/url"
    "strconv"
    "time"

    "github.com/Benni-Math/personal_projects/tree/go-tutorial/go/bittorrent/lib/peers"

    "github.com/jackpal/bencode-go"
)

type bencodeTrackerResp struct {
    Interval    int     `bencode:"interval"`
    Peers       string  `bencode:"peers"`
}

func (t *TorrentFile) buildTrackerURL(peerID [20]byte, port uint16) (string, error) {
    base, err := url.Parse(t.Announce)
}
