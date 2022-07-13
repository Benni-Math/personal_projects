# A simple little BitTorrent client in Go

Based on the [blog post](https://blog.jse.li/posts/torrent/) by Jesse Li.

---

For testing this BitTorrent client, we will be downloading a Debian ISO by
using the tracker (central server) with HTTP address
[http://bttracker.debian.org:6969/](http://bttracker.debian.org:6969/).

## Step 1 - processing .torrent files

Torrent files (denoted by `*.torrent`) are encoded in a format called
**Bencode**, which is roughly similar to JSON. These files contain a couple
different pieces:
 - the URL of the tracker
 - the creation date (UNIX timestamp)
 - the name and size of the file
 - a binary blob containing SHA-1 hashes of each piece

We then want to download each piece (usually between 256KB and 1MB) and compare
them against the SHA-1 pieces (to make sure everything is ok), and then we
assemble the file.

We won't write our own bencode parser - we'll use the one at
github.com/jackpal/bencode-go.

Most of the code for processing .torrent files is located at
`./vendor/torrentfile`.

## Step 2 - Connecting to peers

First, we need to parse through the TorrentFile to get our 'announce' URL which
we use to send a GET query for a list of peers. First, let's describe the list
of peers.

Next up, we need to establish a connection to a peer. This is in three parts
 1. Set TCP connection (standard)
 2. Complete a BitTorrent handshake (files in `./vendor/handshake`)
 3. Exchange messages to download the pieces of the file - `./vendor/p2p`



