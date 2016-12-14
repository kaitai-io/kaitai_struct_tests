<?php
namespace Kaitai\Struct\Tests;

class OpaqueExternalType02ParentTest extends TestCase {
    public function testOpaqueExternalType02Parent() {
        $r = OpaqueExternalType02Parent::fromFile(self::SRC_DIR_PATH . "/term_strz.bin");

        $this->assertEquals('foo', $r->parent->child->s1);
        $this->assertEquals('bar', $r->parent->child->s2);
        $this->assertEquals('|baz@', $r->parent->child->s3->s3);
    }
}
