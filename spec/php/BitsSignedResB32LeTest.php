<?php
// Autogenerated from KST: please remove this line if doing any edits by hand!

namespace Kaitai\Struct\Tests;

class BitsSignedResB32LeTest extends TestCase {
    public function testBitsSignedResB32Le() {
        $r = BitsSignedResB32Le::fromFile(self::SRC_DIR_PATH . '/bits_shift_by_b32_le.bin');

        $this->assertSame(4294967295, $r->a());
    }
}
