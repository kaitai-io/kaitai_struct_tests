<?php
namespace Kaitai\Struct\Tests;

class DebugEnumNameTest extends TestCase {
    public function testDebugEnumName() {
        $r = DebugEnumName::fromFile(self::SRC_DIR_PATH . "/fixed_struct.bin");

        # this test is meaningful only for languages that have --debug and do
        # not save enum type info
    }
}
