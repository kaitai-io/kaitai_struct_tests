<?php
namespace Kaitai\Struct\Tests;

class OpaqueExternalType02ParentTest extends TestCase {
    public function testOpaqueExternalType02Parent() {
        $r = OpaqueExternalType02Parent::fromFile(self::SRC_DIR_PATH . '/term_strz.bin');

        $this->assertSame("foo", $r->parent()->child()->s1());
        $this->assertSame("bar", $r->parent()->child()->s2());
        $this->assertSame("|baz@", $r->parent()->child()->s3()->s3());
    }
}
