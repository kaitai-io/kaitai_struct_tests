<?php
// Autogenerated from KST: please remove this line if doing any edits by hand!

namespace Kaitai\Struct\Tests;

class BitsUnalignedB64BeTest extends TestCase {
    public function testBitsUnalignedB64Be() {
        $r = BitsUnalignedB64Be::fromFile(self::SRC_DIR_PATH . '/process_xor_4.bin');

        $this->assertSame(true, $r->a());
        $this->assertSame(-2776673502979581847, $r->b());
        $this->assertSame(14, $r->c());
    }
}