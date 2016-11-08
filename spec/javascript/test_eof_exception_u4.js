var testHelperThrows = require('testHelperThrows');

// TODO: may be this should throw KaitaiEOFError for uniformity?
testHelperThrows('EofExceptionU4', 'src/term_strz.bin', RangeError);
