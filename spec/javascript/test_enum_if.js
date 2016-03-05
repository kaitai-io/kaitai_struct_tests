var assert = require('assert');
var testHelper = require('testHelper');

testHelper('EnumIf', 'src/if_struct.bin', function(r, EnumIf) {
    assert.equal(r.op1.opcode, EnumIf.Opcodes.A_STRING);
    assert.equal(r.op1.argStr.str, "foo");

    assert.equal(r.op2.opcode, EnumIf.Opcodes.A_TUPLE);
    assert.equal(r.op2.argTuple.num1, 0x42);
    assert.equal(r.op2.argTuple.num2, 0x43);

    assert.equal(r.op3.opcode, EnumIf.Opcodes.A_STRING);
    assert.equal(r.op3.argStr.str, "bar");
});
