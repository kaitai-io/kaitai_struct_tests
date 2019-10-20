var testHelperThrows = require('testHelperThrows');

// TODO: may be this should throw KaitaiEOFError for uniformity?
testHelperThrows('EosExceptionU4', 'src/term_strz.bin', RangeError);
