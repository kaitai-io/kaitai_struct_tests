var assert = require('assert');
var testHelper = require('testHelper');

// FIXME: fix duplication of `test_str_encodings_escaping_to_s.js` (this is a
// copy of the `validateErr` function from there without comments)
function validateErr(expectedEncoding, err) {
  assert(err instanceof RangeError, "expected " + RangeError.name + ", but got " + err);
  assert.strictEqual(err.code, 'ERR_ENCODING_NOT_SUPPORTED');
  assert.strictEqual(err.message, 'The "' + expectedEncoding + '" encoding is not supported');
  return true;
}

function assertUnknownEncoding(expectedEncoding, fn) {
  assert.throws(fn, validateErr.bind(null, expectedEncoding));
}

testHelper('StrEncodingsEscapingEnc', 'src/str_encodings.bin', function(r, StrEncodingsEscapingEnc) {
  assertUnknownEncoding(
    "ASCII\\\\x",
    function() {
      r.str1.v;
    }
  );
  assertUnknownEncoding(
    "UTF-8\\'x",
    function() {
      r.str2.v;
    }
  );
  assertUnknownEncoding(
    "SJIS\\\"x",
    function() {
      r.str3.v;
    }
  );
  assertUnknownEncoding(
    "IBM437\\nx",
    function() {
      r.str4.v;
    }
  );
});
