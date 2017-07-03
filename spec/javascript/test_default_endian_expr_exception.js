var testHelperThrows = require('testHelperThrows');
var KaitaiStream = require('KaitaiStream');

testHelperThrows('DefaultEndianExprException', 'src/endian_expr.bin', KaitaiStream.UndecidedEndiannessError);
