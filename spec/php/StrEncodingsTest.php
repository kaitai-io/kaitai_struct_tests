<?php
namespace Kaitai\Struct\Tests;

class StrEncodingsTest extends TestCase {
    public function testStrEncodings() {
        $r = StrEncodings::fromFile(self::SRC_DIR_PATH . "/str_encodings.bin");

        $this->assertEquals("Some ASCII", $r->str1());
        $this->assertEquals("こんにちは", $r->str2());
        $this->assertEquals("こんにちは", $r->str3());
        $this->assertEquals("░▒▓", $r->str4());
    }
}
