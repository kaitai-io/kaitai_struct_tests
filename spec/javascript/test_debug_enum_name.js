var assert = require('assert');
var testHelper = require('testHelper');

testHelper('DebugEnumName', 'src/fixed_struct.bin', function(r) {
    r._read();
    assert.equal(r._debug.one.enumName, "DebugEnumName.TestEnum1");
    assert.equal(r._debug.arrayOfInts.arr[0].enumName, "DebugEnumName.TestEnum2");
    assert.equal(r.testType._debug.field1.enumName, "DebugEnumName.TestType.InnerEnum1");
    assert.equal(r.testType._debug._m_instanceField, undefined);
    r.testType.instanceField;
    assert.equal(r.testType._debug._m_instanceField.enumName, "DebugEnumName.TestType.InnerEnum2");
});
