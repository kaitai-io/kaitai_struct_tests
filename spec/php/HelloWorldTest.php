<?php
namespace Kaitai\Struct\Tests;

use Kaitai\Struct\Tests\HelloWorld;

class HelloWorldTest extends \PHPUnit_Framework_TestCase {
    const FILE_PATH = __DIR__ . '/../../src/fixed_struct.bin';
    public function testHelloWorld() {
        $r = HelloWorld::fromFile(self::FILE_PATH);
        // Access property through direct call
        $this->assertEquals(0x50, $r->one());
        // Access property through get call
        $this->assertEquals(0x50, $r->one);
    }
}
