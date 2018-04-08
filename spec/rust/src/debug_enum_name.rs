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

pub struct DebugEnumName {
pub struct TestSubtype {
pub struct InnerEnum1 {
pub struct InnerEnum2 {
pub struct TestEnum1 {
pub struct TestEnum2 {
    pub one: ,
    pub arrayOfInts: Vec<>*,
    pub testType: ,
    pub instanceField: ,
    pub field1: ,
    pub field2: u8,
}

impl KaitaiStruct for DebugEnumName {
    fn new<S: KaitaiStream>(stream: &mut S,
                            _parent: &Option<Box<KaitaiStruct>>,
                            _root: &Option<Box<KaitaiStruct>>)
                            -> Result<Self>
        where Self: Sized {
        let mut s = Self {
        }

        impl KaitaiStruct for TestSubtype {
            fn new<S: KaitaiStream>(stream: &mut S,
                                    _parent: &Option<Box<KaitaiStruct>>,
                                    _root: &Option<Box<KaitaiStruct>>)
                                    -> Result<Self>
                where Self: Sized {
                let mut s = Self {
            one: ,
            arrayOfInts: Vec<>*,
            testType: ,
            instanceField: ,
            field1: ,
            field2: 0,
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
        self.one = stream.read_u1()?;
        self.arrayOfInts = [];
        $n = 1;
        for ($i = 0; $i < $n; $i++) {
            self.arrayOfInts[] = stream.read_u1()?;
        }
        self.testType = new debug_enum_name::test_subtype(stream, $this, _root);

        Ok(())
    }
}
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
        self.field1 = stream.read_u1()?;
        self.field2 = stream.read_u1()?;

        Ok(())
    }
    public function instanceField() {
        if (self.instanceField !== null)
            return self.instanceField;
        self.instanceField = ($this->field2() & 15);
        return self.instanceField;
    }
}
const ENUM_VALUE_67 = 67;
}
const ENUM_VALUE_11 = 11;
}
const ENUM_VALUE_80 = 80;
}
const ENUM_VALUE_65 = 65;
}
