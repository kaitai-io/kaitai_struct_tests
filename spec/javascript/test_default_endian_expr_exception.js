var testHelperThrows = require('testHelperThrows');

testHelperThrows('DefaultEndianExprException', 'src/endian_expr.bin', KaitaiUndecidedEndiannessError);
