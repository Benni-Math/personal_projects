# A simple little BitTorrent client in Go

Based on the [blog post](https://blog.jse.li/posts/torrent/) by Jesse Li.

To build, use `go build -o torrent-client main.go` and to run, get a .torrent
file and use `./torrent-client <your-torrentfile-name>.torrent <target-name>`.

---

For testing this BitTorrent client, we will be downloading a Debian ISO by
using the tracker (central server) with HTTP address
[http://bttracker.debian.org:6969/](http://bttracker.debian.org:6969/).

The file `walkthrough.go` contains the first bits of a walkthrough through the
code which is covered in the article. The vendor folder contains the fleshed
out code.

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

Once we complete the handshake, we wait for an unchoke message to be sent from
the peer and then we can finally start requesting the pieces of our download.

## Step 3 - Downloading the torrent

This next step is the most difficult one, since we have to deal with
concurrency and state management, but thankfully Go has our back - Goroutines
and channels make it surprisingly easy to implement concurrency, and the
general simplicity of Go makes it easier to understand state (so long as we
don't complicate things for ourselves).

The first important piece is the `Torrent.Download` method in `p2p.go` - this
function creates two channels
 1. A buffered channel, called the `workQueue` which keeps track of the pieces
 and distributes them among the threads for download
 2. The results channel, which sends the finished downloads back to main

 From here we start spawning goroutines running the
 `Torrent.startDownloadWorker` method (doing this for each peer). Notice the
 high level of concurrency -- we are handling multiple different downloads,
 which we don't want to repeat, among multiple different peers, but the
 buffered channel, along with the single result channel make this easy. The
 final piece of this method assembles all of the pieces by drawing from the
 result channel and copies them all onto a buffer, which we then return.

 To keep track of the download progress, we use a couple different structs by
 the name of `pieceWork`, `pieceResult`, and `pieceProgress`.

