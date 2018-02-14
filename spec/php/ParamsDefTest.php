<?php
namespace Kaitai\Struct\Tests;

class ParamsDefTest extends TestCase {
    public function testParamsDef() {
        $io = new \Kaitai\Struct\Stream(fopen(self::SRC_DIR_PATH . "/term_strz.bin", "rb"));
        $r = new ParamsDef(5, true, $io);

        $this->assertEquals("foo|b", $r->buf);
        $this->assertEquals(0x61, $r->trailer);
    }
}
