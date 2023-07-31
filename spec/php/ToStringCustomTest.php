<?php
namespace Kaitai\Struct\Tests;

class ToStringCustomTest extends TestCase {
    public function testToStringCustom() {
        $r = ToStringCustom::fromFile(self::SRC_DIR_PATH . '/term_strz.bin');

        $this->assertSame("s1 = foo, s2 = bar", strval($r));
    }
}
