<?php
namespace Kaitai\Struct\Tests;

class DefaultEndianExprExceptionTest extends TestCase {
    /**
     * @expectedException \RuntimeException
     * @expectedExceptionMessage Unable to decide on endianness
     */
    public function testDefaultEndianExprException() {
        DefaultEndianExprException::fromFile(self::SRC_DIR_PATH . "/endian_expr.bin");
    }
}
