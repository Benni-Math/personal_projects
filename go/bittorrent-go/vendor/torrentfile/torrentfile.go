package torrentfile

import (
	"bytes"	
    "crypto/rand"
    "crypto/sha1"
    "fmt"
    "os"

    "p2p"

    "github.com/jackpal/bencode-go"
)