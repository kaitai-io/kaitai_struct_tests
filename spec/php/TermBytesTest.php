<?php
// Autogenerated from KST: please remove this line if doing any edits by hand!

namespace Kaitai\Struct\Tests;

class TermBytesTest extends TestCase {
    public function testTermBytes() {
        $r = TermBytes::fromFile(self::SRC_DIR_PATH . '/term_strz.bin');

        $this->assertSame("\x66\x6F\x6F", $r->s1());
        $this->assertSame("\x62\x61\x72", $r->s2());
        $this->assertSame("\x7C\x62\x61\x7A\x40", $r->s3());
    }
}
