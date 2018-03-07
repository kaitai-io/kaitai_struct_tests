var testHelperThrows = require('testHelperThrows');
var KaitaiStream = require('kaitai-struct/KaitaiStream');

testHelperThrows('EofExceptionBytes', 'src/term_strz.bin', KaitaiStream.EOFError);
