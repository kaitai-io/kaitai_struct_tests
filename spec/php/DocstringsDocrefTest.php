<?php
namespace Kaitai\Struct\Tests;

class DocstringsDocrefTest extends TestCase {
    public function testDocstringsDocref() {
        $r = DocstringsDocref::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");
        $this->markTestAsNotRisky();
    }
}
