<?php
namespace Kaitai\Struct\Tests;

class FixedStructTest extends TestCase {
    public function testFixedStruct() {
        $r = FixedStruct::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        $this->assertEquals(255, $r->hdr()->uint8());
        $this->assertEquals(65535, $r->hdr()->uint16());
        $this->assertEquals(4294967295, $r->hdr()->uint32());
        // PHP does not have unsigned integers, it has only signed integers in
        // the range [PHP_INT_MIN, PHP_INT_MAX], so to represent 2^64 - 1 we
        // need to use an integer with the same internal binary representation, i.e. -1.
        $this->assertEquals(-1, $r->hdr()->uint64());

        $this->assertEquals(-1, $r->hdr()->sint8());
        $this->assertEquals(-1, $r->hdr()->sint16());
        $this->assertEquals(-1, $r->hdr()->sint32());
        $this->assertEquals(-1, $r->hdr()->sint64());

        $this->assertEquals(66, $r->hdr()->uint16le());
        $this->assertEquals(66, $r->hdr()->uint32le());
        $this->assertEquals(66, $r->hdr()->uint64le());

        $this->assertEquals(-66, $r->hdr()->sint16le());
        $this->assertEquals(-66, $r->hdr()->sint32le());
        $this->assertEquals(-66, $r->hdr()->sint64le());

        $this->assertEquals(66, $r->hdr()->uint16be());
        $this->assertEquals(66, $r->hdr()->uint32be());
        $this->assertEquals(66, $r->hdr()->uint64be());

        $this->assertEquals(-66, $r->hdr()->sint16be());
        $this->assertEquals(-66, $r->hdr()->sint32be());
        $this->assertEquals(-66, $r->hdr()->sint64be());
    }
}
