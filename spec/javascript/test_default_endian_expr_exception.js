var testHelperThrows = require('testHelperThrows');
var KaitaiStream = require('kaitai-struct/KaitaiStream');

testHelperThrows('DefaultEndianExprException', 'src/endian_expr.bin', KaitaiStream.UndecidedEndiannessError);
