var assert = require('assert');
var testHelper = require('testHelper');

testHelper('IfStruct', 'src/if_struct.bin', function(r) {
    assert.equal(r.op1.opcode, 0x53);
    assert.equal(r.op1.argStr.str, "foo");

    assert.equal(r.op2.opcode, 0x54);
    assert.equal(r.op2.argTuple.num1, 0x42);
    assert.equal(r.op2.argTuple.num2, 0x43);

    assert.equal(r.op3.opcode, 0x53);
    assert.equal(r.op3.argStr.str, "bar");
})
