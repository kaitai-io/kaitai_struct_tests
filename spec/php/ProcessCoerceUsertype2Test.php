<?php
namespace Kaitai\Struct\Tests;

class ProcessCoerceUsertype2Test extends TestCase {
    public function testProcessCoerceUsertype2() {
        $r = ProcessCoerceUsertype2::fromFile(self::SRC_DIR_PATH . "/process_coerce_bytes.bin");

        $this->assertEquals(0, $r->records[0]->flag);
        $this->assertEquals(0x41414141, $r->records[0]->buf->value);
        $this->assertEquals(1, $r->records[1]->flag);
        $this->assertEquals(0x42424242, $r->records[1]->buf->value);
    }
}
