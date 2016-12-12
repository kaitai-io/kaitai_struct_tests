<?php
namespace Kaitai\Struct\Tests;

class TypeTernaryTest extends TestCase {
    public function testTypeTernary() {
        $r = TypeTernary::fromFile(self::SRC_DIR_PATH . "/term_strz.bin");

        $this->assertEquals(0x65, $r->dif->value);
    }
}
