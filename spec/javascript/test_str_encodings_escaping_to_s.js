var assert = require('assert');
var testHelper = require('testHelper');

function validateErr(expectedEncoding, err) {
  if (err instanceof RangeError) {
    // See https://nodejs.org/docs/latest-v20.x/api/errors.html#err_encoding_not_supported
    assert.strictEqual(err.code, 'ERR_ENCODING_NOT_SUPPORTED');

    // Node.js 20: 'The "unknown" encoding is not supported'
    // Chrome 123: "Failed to construct 'TextDecoder': The encoding label provided ('unknown') is invalid."
    // Firefox 124: "TextDecoder constructor: The given encoding 'unknown' is not supported."
    //
    // Since we only run tests on Node.js at the time of writing, we don't need to
    // support all the possible message texts thrown by other JS environments (see
    // above) - let's focus on Node.js.
    assert.strictEqual(err.message, 'The "' + expectedEncoding + '" encoding is not supported');
  } else if (Object.getPrototypeOf(err) === Error.prototype) {
    // In Node.js 10 and below, a plain Error with a different message is thrown
    // instead of a RangeError, so we'll accept that as well.
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

testHelper('StrEncodingsEscapingToS', 'src/str_encodings.bin', function(r, StrEncodingsEscapingToS_) {
  assertUnknownEncoding(
    "ASCII\\\\x",
    function() {
      r.str1;
    }
  );
  assertUnknownEncoding(
    "UTF-8\\'x",
    function() {
      r.str2;
    }
  );
  assertUnknownEncoding(
    "SJIS\\\"x",
    function() {
      r.str3;
    }
  );
  assertUnknownEncoding(
    "IBM437\\nx",
    function() {
      r.str4;
    }
  );
});
