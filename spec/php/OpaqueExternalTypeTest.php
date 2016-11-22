<?php
namespace Kaitai\Struct\Tests;

class OpaqueExternalTypeTest extends TestCase {
    public function testOpaqueExternalType() {
        $r = OpaqueExternalType::fromFile(self::SRC_DIR_PATH . "/term_strz.bin");

        $this->assertEquals('foo', $r->one->s1);
        $this->assertEquals('bar', $r->one->s2);
        $this->assertEquals('|baz@', $r->one->s3);
    }
}
