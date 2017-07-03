var testHelperThrows = require('testHelperThrows');
var KaitaiStream = require('KaitaiStream');

testHelperThrows('EofExceptionBytes', 'src/term_strz.bin', KaitaiStream.EOFError);
