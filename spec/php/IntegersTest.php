<?php

namespace Kaitai\Struct\Tests;

class IntegersTest extends TestCase {
    public function testIntegers() {
        $r = Integers::fromFile(self::SRC_DIR_PATH . '/fixed_struct.bin');

        $this->assertEquals(255, $r->uint8());
        $this->assertEquals(65535, $r->uint16());
        $this->assertEquals(4294967295, $r->uint32());
        $this->assertEquals(-1, $r->uint64());
        $this->assertEquals(-1, $r->sint8());
        $this->assertEquals(-1, $r->sint16());
        $this->assertEquals(-1, $r->sint32());
        $this->assertEquals(-1, $r->sint64());
        $this->assertEquals(66, $r->uint16le());
        $this->assertEquals(66, $r->uint32le());
        $this->assertEquals(66, $r->uint64le());
        $this->assertEquals(-66, $r->sint16le());
        $this->assertEquals(-66, $r->sint32le());
        $this->assertEquals(-66, $r->sint64le());
        $this->assertEquals(66, $r->uint16be());
        $this->assertEquals(66, $r->uint32be());
        $this->assertEquals(66, $r->uint64be());
        $this->assertEquals(-66, $r->sint16be());
        $this->assertEquals(-66, $r->sint32be());
        $this->assertEquals(-66, $r->sint64be());
    }
}
