<?php
namespace Kaitai\Struct\Tests;

class FixedContentsTest extends TestCase {
    public function testFixedContents() {
        $r = FixedContents::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");
        $this->markTestAsNotRisky();
    }
}
