var assert = require('assert');
var testHelper = require('testHelper');

testHelper('RepeatEosU4', 'src/repeat_eos_struct.bin', function(r) {
    assert.deepEqual([0, 0x42, 0x42, 0x815], r.numbers);
});
