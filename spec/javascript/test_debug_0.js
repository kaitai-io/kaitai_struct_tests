var assert = require('assert');
var testHelper = require('testHelper');

testHelper('Debug0', 'src/fixed_struct.bin', function(r) {
  r._read();

  assert.strictEqual(r.one, 80);
  assert.deepStrictEqual(r.arrayOfInts, [65, 67, 75]);
  assert.strictEqual(r._unnamed2, 45);

  assert.deepStrictEqual(r._debug, {
      "one": {
          "start": 0,
          "ioOffset": 0,
          "end": 1
      },
      "arrayOfInts": {
          "start": 1,
          "ioOffset": 0,
          "arr": [
              {
                  "start": 1,
                  "ioOffset": 0,
                  "end": 2
              },
              {
                  "start": 2,
                  "ioOffset": 0,
                  "end": 3
              },
              {
                  "start": 3,
                  "ioOffset": 0,
                  "end": 4
              }
          ],
          "end": 4
      },
      "_unnamed2": {
          "start": 4,
          "ioOffset": 0,
          "end": 5
      }
  });
});
