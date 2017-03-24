<?php
namespace Kaitai\Struct\Tests;

class DocstringsTest extends TestCase {
    public function testDocstrings() {
        $r = Docstrings::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");
        $this->markTestAsNotRisky();
    }
}
