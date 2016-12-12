<?php
namespace Kaitai\Struct\Tests;

class TypeTernaryOpaqueTest extends TestCase {
    public function testTypeTernaryOpaque() {
        $r = TypeTernaryOpaque::fromFile(self::SRC_DIR_PATH . "/term_strz.bin");

        $this->assertEquals('foo', $r->dif->s1);
        $this->assertEquals('bar', $r->dif->s2);
        $this->assertEquals('|baz@', $r->dif->s3);
    }
}
