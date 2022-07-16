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
    ID: MsgID,
    Payload: [u8; 20],
}

