// Read about different enum types
enum MsgID {
    Choke,
    Unchoke,
    Interested,
    NotInterested,
    Have,
    Bitfield,
    Request,
    Piece,
    Cancel,
}

pub struct Message {
    id: MsgID,
    payload: Vec<u8>,
}

// Could simplify this into one 'format' or 'process'
// by using a match
impl Message {
    pub fn format_request(index: usize, begin: usize, length: u8) -> Self {
        let mut payload = [0u8; 12];
        // Read BigEndian
        Message { id: MsgID::Request, payload: payload }
    }

    pub fn format_have(index: usize) {
       let payload = //read [4]byte from BigEndian
       
       Message { id: MsgID::Have, payload: payload }
    }

    // Might have an issue with buf referencing/borrowing/lifetime..
    //                      Fix this type--v
    pub fn parse_piece(&self, index: usize, buf Vec<u8>)
        -> Result<usize, Error /* Need error type */>
    {
        // TODO: make these if-statements into one match-statement?
        if self.id != MsgID::Piece { /* Throw error */ }
        if self.payload.length < 8 { /* Throw error */ }

        let parsedIndex = ;// Read first 4 as big-endian
        if parsedIndex != index { /* Throw error */ }

        let begin = ;// Read [4:8] as big-endian
        if begin >= buf.length { /* Throw error */ }

        let data = self.payload[8:]; // How to slice a Vec?
        if begin+data.length > buf.length { /* Throw error */ }

        // copy data into buf[begin:]
        Ok(index)
    }

    pub fn parse_have(&self) -> Result<usize, Error> {
        if self.id != MsgID::Have { /* Throw error */ }
        if self.payload.length != 4 { /* Throw error */ }
        
        let index = ;// read payload as big-endian
        Ok(index)
    }

    // Serialize a message into a buffer of the form
    // <length prefix><message ID><payload>
    // (interpret nil as keep alive? --> Option<MsgID> or MsgID::KeepAlive
    pub fn serialize(&self) -> Vec<u8> {
        
    }

    // Read parses a message from a stream
    // What is the Rust equivalent of io.Reader?
    pub fn read() -> Result<Self> {
    }
}

// MsgID to String
// Message to String
impl Display for Message {}

