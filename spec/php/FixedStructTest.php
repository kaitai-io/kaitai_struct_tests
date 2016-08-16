<?php
namespace Kaitai\Struct\Tests;

class FixedStructTest extends TestCase {
    public function testFixedStruct() {
        $this->markTestIncomplete();
/*
        FixedStruct r = FixedStruct::fromFile(self::SRC_DIR_PATH . "fixed_struct.bin");

        $this->assertEquals(r.hdr().uint8(), 255);
        $this->assertEquals(r.hdr().uint16(), 65535);
        $this->assertEquals(r.hdr().uint32(), 4294967295L);
        //$this->assertEquals(r.hdr().uint64(), 18446744073709551615);
        $this->assertEquals(r.hdr().uint64(), 0xFFFFFFFFFFFFFFFFL);

        $this->assertEquals(r.hdr().sint8(), -1);
        $this->assertEquals(r.hdr().sint16(), -1);
        $this->assertEquals(r.hdr().sint32(), -1);
        $this->assertEquals(r.hdr().sint64(), -1);

        $this->assertEquals(r.hdr().uint16le(), 66);
        $this->assertEquals(r.hdr().uint32le(), 66);
        $this->assertEquals(r.hdr().uint64le(), 66);

        $this->assertEquals(r.hdr().sint16le(), -66);
        $this->assertEquals(r.hdr().sint32le(), -66);
        $this->assertEquals(r.hdr().sint64le(), -66);

        $this->assertEquals(r.hdr().uint16be(), 66);
        $this->assertEquals(r.hdr().uint32be(), 66);
        $this->assertEquals(r.hdr().uint64be(), 66);

        $this->assertEquals(r.hdr().sint16be(), -66);
        $this->assertEquals(r.hdr().sint32be(), -66);
        $this->assertEquals(r.hdr().sint64be(), -66);
*/    
    }
}
