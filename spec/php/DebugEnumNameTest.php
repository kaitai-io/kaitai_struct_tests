<?php
namespace Kaitai\Struct\Tests;

class DebugEnumNameTest extends TestCase {
    public function testDebugEnumName() {
        $r = DebugEnumName::fromFile(self::SRC_DIR_PATH . '/fixed_struct.bin');

        // --debug implies --no-auto-read
        $r->_read();

        $this->assertSame(\Kaitai\Struct\Tests\DebugEnumName\TestEnum1::ENUM_VALUE_80, $r->one());
        $this->assertSame(\Kaitai\Struct\Tests\DebugEnumName\TestEnum2::ENUM_VALUE_65, $r->arrayOfInts()[0]);
        $this->assertSame(\Kaitai\Struct\Tests\DebugEnumName\TestSubtype\InnerEnum1::ENUM_VALUE_67, $r->testType()->field1());
        $this->assertSame(\Kaitai\Struct\Tests\DebugEnumName\TestSubtype\InnerEnum2::ENUM_VALUE_11, $r->testType()->instanceField());
    }
}
