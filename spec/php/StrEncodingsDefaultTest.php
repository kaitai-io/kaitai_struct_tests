<?php
namespace Kaitai\Struct\Tests;

class StrEncodingsDefaultTest extends TestCase {
    public function testStrEncodingsDefault() {
        $r = StrEncodingsDefault::fromFile(self::SRC_DIR_PATH . "/str_encodings.bin");

        $this->assertEquals('Some ASCII', $r->str1);
        $this->assertEquals('こんにちは', $r->rest->str2);
        $this->assertEquals('こんにちは', $r->rest->str3);
        $this->assertEquals('░▒▓', $r->rest->str4);
    }
}
