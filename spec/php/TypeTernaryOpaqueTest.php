<?php
namespace Kaitai\Struct\Tests;

class TypeTernaryOpaqueTest extends TestCase {
    public function testTypeTernaryOpaque() {
        $r = TypeTernaryOpaque::fromFile(self::SRC_DIR_PATH . "/term_strz.bin");

        $this->assertSame('foo', $r->dif->s1);
        $this->assertSame('bar', $r->dif->s2);
        $this->assertSame('|baz@', $r->dif->s3);
    }
}
