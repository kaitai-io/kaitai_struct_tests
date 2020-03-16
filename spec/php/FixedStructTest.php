<?php
namespace Kaitai\Struct\Tests;

class FixedStructTest extends TestCase {
    public function testFixedStruct() {
        $r = FixedStruct::fromFile(self::SRC_DIR_PATH . '/fixed_struct.bin');

        $this->assertSame(255, $r->hdr()->uint8());
        $this->assertSame(65535, $r->hdr()->uint16());
        $this->assertSame(4294967295, $r->hdr()->uint32());
        // PHP does not have unsigned integers, it has only signed integers in
        // the range [PHP_INT_MIN, PHP_INT_MAX], so to represent 2^64 - 1 we
        // need to use an integer with the same internal binary representation, i.e. -1.
        $this->assertSame(-1, $r->hdr()->uint64());

        $this->assertSame(-1, $r->hdr()->sint8());
        $this->assertSame(-1, $r->hdr()->sint16());
        $this->assertSame(-1, $r->hdr()->sint32());
        $this->assertSame(-1, $r->hdr()->sint64());

        $this->assertSame(66, $r->hdr()->uint16le());
        $this->assertSame(66, $r->hdr()->uint32le());
        $this->assertSame(66, $r->hdr()->uint64le());

        $this->assertSame(-66, $r->hdr()->sint16le());
        $this->assertSame(-66, $r->hdr()->sint32le());
        $this->assertSame(-66, $r->hdr()->sint64le());

        $this->assertSame(66, $r->hdr()->uint16be());
        $this->assertSame(66, $r->hdr()->uint32be());
        $this->assertSame(66, $r->hdr()->uint64be());

        $this->assertSame(-66, $r->hdr()->sint16be());
        $this->assertSame(-66, $r->hdr()->sint32be());
        $this->assertSame(-66, $r->hdr()->sint64be());
    }
}
