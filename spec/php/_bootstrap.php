<?php
require __DIR__ . '/../../../runtime/php/lib/Kaitai/Struct/Struct.php';
require __DIR__ . '/../../../runtime/php/lib/Kaitai/Struct/Stream.php';

spl_autoload_register(function ($class) {
    $kt = "Kaitai\\Struct\\Tests\\";
    if (substr($class, 0, strlen($kt)) === $kt) {
        $testName = substr($class, strlen($kt));
        require __DIR__ . "/../../compiled/php/$testName.php";
    }
});
