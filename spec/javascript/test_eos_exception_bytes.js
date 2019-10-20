var testHelperThrows = require('testHelperThrows');
var KaitaiStream = require('kaitai-struct/KaitaiStream');

testHelperThrows('EosExceptionBytes', 'src/term_strz.bin', KaitaiStream.EOFError);
