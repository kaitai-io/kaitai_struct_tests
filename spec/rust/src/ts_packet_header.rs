// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

#![allow(unused_parens)]
use std::option::Option;
use std::boxed::Box;
use std::io::Result;
use std::default::Default;
use kaitai_struct::KaitaiStream;
use kaitai_struct::KaitaiStruct;


/*
 * describes the first 4 header bytes of a TS Packet header
 */
#[derive(Default)]
pub struct TsPacketHeader {
    pub syncByte: u8,
    pub transportErrorIndicator: bool,
    pub payloadUnitStartIndicator: bool,
    pub transportPriority: bool,
    pub pid: u64,
    pub transportScramblingControl: u64,
    pub adaptationFieldControl: Box<TsPacketHeader__AdaptationFieldControlEnum>,
    pub continuityCounter: u64,
    pub tsPacketRemain: Vec<u8>,
}

impl KaitaiStruct for TsPacketHeader {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s: Self = Default::default();

        s.stream = stream;
        s.read(stream, _parent, _root)?;

        Ok(s)
    }


    fn read<S: KaitaiStream>(&mut self,
                             stream: &mut S,
                             _parent: &Option<Box<KaitaiStruct>>,
                             _root: &Option<Box<KaitaiStruct>>)
                             -> Result<()>
        where Self: Sized {
        self.syncByte = self.stream.read_u1()?;
        self.transportErrorIndicator = self.stream.read_bits_int(1)? != 0;
        self.payloadUnitStartIndicator = self.stream.read_bits_int(1)? != 0;
        self.transportPriority = self.stream.read_bits_int(1)? != 0;
        self.pid = self.stream.read_bits_int(13)?;
        self.transportScramblingControl = self.stream.read_bits_int(2)?;
        self.adaptationFieldControl = self.stream.read_bits_int(2)?;
        self.continuityCounter = self.stream.read_bits_int(4)?;
        self.stream.alignToByte();
        self.tsPacketRemain = self.stream.read_bytes(184)?;
    }
}

impl TsPacketHeader {
}
enum TsPacketHeader__AdaptationFieldControlEnum {
    RESERVED,
    PAYLOAD_ONLY,
    ADAPTATION_FIELD_ONLY,
    ADAPTATION_FIELD_AND_PAYLOAD,
}
