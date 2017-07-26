<?php
namespace Kaitai\Struct\Tests;

class ParamsCallShortTest extends TestCase {
    public function testParamsCallShort() {
        $r = ParamsCallShort::fromFile(self::SRC_DIR_PATH . "/term_strz.bin");

        $this->assertEquals("foo|b", $r->buf1->body);
        $this->assertEquals("ar|ba", $r->buf2->body);
        $this->assertEquals(0x7a, $r->buf2->trailer);
    }
}
