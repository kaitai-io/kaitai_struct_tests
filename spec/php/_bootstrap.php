<?php
require __DIR__ . '/../../../runtime/php/lib/Kaitai/Struct/Struct.php';
require __DIR__ . '/../../../runtime/php/lib/Kaitai/Struct/Stream.php';

spl_autoload_register(function ($class) {
    /*
    $kt = "Kaitai\\Tests\\";
    if (substr($name, 0, strlen($kt)) === $kt) {
        $testName = substr($name, strlen($kt));
    */
    $classes = [
        'HelloWorld'
    ];
    if (in_array($class, $classes, true)) {
        require __DIR__ . "/../../compiled/php/$class.php";
    }
    //}
});
