// TODO: rethink project structure
mod client;
mod p2p;
mod tracker;

use self::p2p::Torrent;

// Need to find a bencode parser
// (or write my own...)

const PORT: u64 = 6881; // Might need to do something slightly different

#[derive(Debug)]
pub struct TorrentFile {
    announce: &str,
    info_hash: [u8; 20],
    piece_hashes: Vec<[u8; 20]>,
    length: usize,
    name: &str,
}

impl TorrentFile {
    // Need to setup error types...
    pub fn download_to_file(&self, path: &str) -> Option<()> {
        // Setup random peerID (possible Error?)
        let peerID = [0u8; 20]; // bad way to initialize?

        // TODO: could probably use a functional 'piping'
        // approach here, with and_then(), something like
        //      self.toTorrent().download().writeTo(path)?;
        //              ^`tracker::request_peers` in here

        // Getting the list of peers
        // Torrent is an extension of TorrentFile
        // Figure out how to 'unroll' self nicely
        let torrent: p2p::Torrent = {};
        // Better approach than if let (not unwrap)
        if let Ok(peers) = tracker::request_peers(&self, peerID, PORT);
        // Error handlimg

        // Start download
        match
    }

    pub fn open(path: &str) -> Result<TorrentFile, std::io::Error> {
        // parser comes in here
    }
}

