<?php
namespace Kaitai\Struct\Tests;

class NavParentVsValueInstTest extends TestCase {
    public function testNavParentVsValueInst() {
        $r = NavParentVsValueInst::fromFile(self::SRC_DIR_PATH . "/term_strz.bin");

        $this->assertEquals('foo', $r->s1);
    }
}
