var assert = require('assert');
var testHelper = require('testHelper');

testHelper('DebugEnumName', 'src/fixed_struct.bin', function(r) {
    r._read();
    assert.strictEqual(r._debug.one.enumName, "DebugEnumName.TestEnum1");
    assert.strictEqual(r._debug.arrayOfInts.arr[0].enumName, "DebugEnumName.TestEnum2");
    assert.strictEqual(r.testType._debug.field1.enumName, "DebugEnumName.TestSubtype.InnerEnum1");
    assert.strictEqual(r.testType._debug._m_instanceField, undefined);
    r.testType.instanceField;
    assert.strictEqual(r.testType._debug._m_instanceField.enumName, "DebugEnumName.TestSubtype.InnerEnum2");
});
