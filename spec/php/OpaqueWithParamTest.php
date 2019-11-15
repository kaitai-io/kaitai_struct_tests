<?php
namespace Kaitai\Struct\Tests;

class OpaqueWithParamTest extends TestCase {
    public function testOpaqueWithParam() {
        $r = OpaqueWithParam::fromFile(self::SRC_DIR_PATH . "/term_strz.bin");

        $this->assertEquals('foo|b', $r->one->buf);
        $this->assertEquals(0x61, $r->one->trailer);
    }
}
