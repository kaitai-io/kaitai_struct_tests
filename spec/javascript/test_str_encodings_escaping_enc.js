var assert = require('assert');
var testHelper = require('testHelper');

// FIXME: fix duplication of `test_str_encodings_escaping_to_s.js` (this is a
// copy of the `validateErr` function from there without comments)
function validateErr(expectedEncoding, err) {
  if (err instanceof RangeError) {
    assert.strictEqual(err.code, 'ERR_ENCODING_NOT_SUPPORTED');
    assert.strictEqual(err.message, 'The "' + expectedEncoding + '" encoding is not supported');
  } else if (Object.getPrototypeOf(err) === Error.prototype) {
    var regex = /^Encoding not recognized: '(.*)' \(searched as: '.*'\)$/;
    var match = err.message.match(regex);
    assert.ok(match !== null, "message [" + err.message + "] does not match regex " + regex);
    assert.strictEqual(match[1], expectedEncoding);
  } else {
    assert.fail("expected " + RangeError.name + " or " + Error.name + ", but got " + err);
  }
  return true;
}

function assertUnknownEncoding(expectedEncoding, fn) {
  assert.throws(fn, validateErr.bind(null, expectedEncoding));
}

testHelper('StrEncodingsEscapingEnc', 'src/str_encodings.bin', function(r, StrEncodingsEscapingEnc_) {
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
