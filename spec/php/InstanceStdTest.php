<?php
namespace Kaitai\Struct\Tests;

class InstanceStdTest extends TestCase {
    public function testInstanceStd() {
        $r = InstanceStd::fromFile(self::SRC_DIR_PATH . "/str_encodings.bin");

        $this->assertEquals("Some ", $r->header());
    }
}
