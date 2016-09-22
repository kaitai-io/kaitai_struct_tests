<?php
namespace Kaitai\Struct\Tests;

class InstanceUserArrayTest extends TestCase {
    public function testInstanceUserArray() {
        $r = InstanceUserArray::fromFile(self::SRC_DIR_PATH . "/instance_std_array.bin");

        $this->assertEquals(0x10, $r->ofs);
        $this->assertEquals(3, $r->qtyEntries);
        $this->assertEquals(4, $r->entrySize);

        $this->assertEquals(3, count($r->userEntries));
        $this->assertEquals(0x1111, $r->userEntries[0]->word1);
        $this->assertEquals(0x1111, $r->userEntries[0]->word2);
        $this->assertEquals(0x2222, $r->userEntries[1]->word1);
        $this->assertEquals(0x2222, $r->userEntries[1]->word2);
        $this->assertEquals(0x3333, $r->userEntries[2]->word1);
        $this->assertEquals(0x3333, $r->userEntries[2]->word2);
    }
}
