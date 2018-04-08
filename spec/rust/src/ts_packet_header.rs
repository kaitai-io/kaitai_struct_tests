// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

use std::{
    option::Option,
    boxed::Box,
    io::Result
};

use kaitai_struct::{
    KaitaiStream,
    KaitaiStruct
};

pub struct TsPacketHeader {
pub struct AdaptationFieldControlEnum {
    pub syncByte: u8,
    pub transportErrorIndicator: bool,
    pub payloadUnitStartIndicator: bool,
    pub transportPriority: bool,
    pub pid: u64,
    pub transportScramblingControl: u64,
    pub adaptationFieldControl: ,
    pub continuityCounter: u64,
    pub tsPacketRemain: String,
}

impl KaitaiStruct for TsPacketHeader {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
            syncByte: 0,
            transportErrorIndicator: bool,
            payloadUnitStartIndicator: bool,
            transportPriority: bool,
            pid: u64,
            transportScramblingControl: u64,
            adaptationFieldControl: ,
            continuityCounter: u64,
            tsPacketRemain: String,

/**
 * describes the first 4 header bytes of a TS Packet header
 */
        };

        s.read(stream, _parent, _root)?;

        Ok(s)
    }

    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.syncByte = stream.read_u1()?;
        self.transportErrorIndicator = stream->readBitsInt(1) != 0;
        self.payloadUnitStartIndicator = stream->readBitsInt(1) != 0;
        self.transportPriority = stream->readBitsInt(1) != 0;
        self.pid = stream->readBitsInt(13);
        self.transportScramblingControl = stream->readBitsInt(2);
        self.adaptationFieldControl = stream->readBitsInt(2);
        self.continuityCounter = stream->readBitsInt(4);
        stream->alignToByte();
        self.tsPacketRemain = stream->readBytes(184);

        Ok(())
    }
}
const RESERVED = 0;
const PAYLOAD_ONLY = 1;
const ADAPTATION_FIELD_ONLY = 2;
const ADAPTATION_FIELD_AND_PAYLOAD = 3;
}
