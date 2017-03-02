var assert = require('assert');
var testHelper = require('testHelper');

testHelper('Debug0', 'src/fixed_struct.bin', function(r) {
    r._read();

    assert.equal(r.one, 80);
    assert.equal(r.arrayOfInts.toString(), [65, 67, 75].toString());
    assert.equal(r._unnamed2, 45);

    var actual = JSON.stringify(r._debug, (k,v) => k === '_root' || k === '_parent' || k === '_io' ? undefined : v, 4);
    assert.equal(actual, `{
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
}`);
});
