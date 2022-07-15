package torrentfile

import (
	"bytes"	
    "crypto/rand"
    "crypto/sha1"
    "fmt"
    "os"

    "github.com/Benni-Math/personal_projects/tree/go-tutorial/go/bittorrent/lib/p2p"

    "github.com/jackpal/bencode-go"
)

// Port to listen on
const Port uint16 = 6881

// Torrentfile encodes the metdata from a .torrent file
type TorrentFile struct {
    Announce    string
    infoHash    [20]byte
    PieceHashes [][20]byte
    PieceLength int
    Length      int
    Name        string
}

// Type from jackpal's package -- output of bencode parse
type bencodeInfo struct {
    Pieces      string  `bencode:"pieces"`
    PieceLength int     `bencode:"piece length"`
    Length      int     `bencode:"length"`
    Name        string  `bendcode:"name"`
}

type bencodeTorrent struct struct {
    Announce    string      `bencode:"announce"`
    Info        bencodeInfo `bencode:"info"`
}

// Main function for downloading (calls p2p)

