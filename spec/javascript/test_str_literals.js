var assert = require('assert');
var testHelper = require('testHelper');

testHelper('StrLiterals', 'src/fixed_struct.bin', function(r, StrLiterals_) {
  function strToArr(s) {
    var r = [];
    for (var i = 0; i < s.length; i++)
      r.push(s.charCodeAt(i));
    return r;
  }
  assert.deepStrictEqual(strToArr(r.complexStr), [0, 1, 2, 7, 8, 10, 13, 9, 11, 12, 27, 61, 7, 10, 36, 9787]);
  assert.deepStrictEqual(strToArr(r.doubleQuotes), [34, 34, 34]);
  assert.deepStrictEqual(strToArr(r.backslashes), [92, 92, 92]);
  assert.deepStrictEqual(strToArr(r.octalEatup), [0, 50, 50]);
  assert.deepStrictEqual(strToArr(r.octalEatup2), [2, 50]);
});
