<?php
namespace Kaitai\Struct\Tests;

class OpaqueExternalTypeTest extends TestCase {
    public function testOpaqueExternalType() {
        $r = OpaqueExternalType::fromFile(self::SRC_DIR_PATH . "/term_strz.bin");

        $this->assertSame('foo', $r->one->s1);
        $this->assertSame('bar', $r->one->s2);
        $this->assertSame('|baz@', $r->one->s3);
    }
}
