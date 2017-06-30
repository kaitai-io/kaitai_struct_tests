<?php
namespace Kaitai\Struct\Tests;

class ProcessCoerceBytesTest extends TestCase {
    public function testProcessCoerceBytes() {
        $r = ProcessCoerceBytes::fromFile(self::SRC_DIR_PATH . "/process_coerce_bytes.bin");

        $this->assertEquals(0, $r->records[0]->flag);
        $this->assertEquals("AAAA", $r->records[0]->buf);
        $this->assertEquals(1, $r->records[1]->flag);
        $this->assertEquals("BBBB", $r->records[1]->buf);
    }
}
