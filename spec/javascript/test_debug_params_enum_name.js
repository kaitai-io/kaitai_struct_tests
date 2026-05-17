var assert = require('assert');
var testHelper = require('testHelper');

testHelper('DebugParamsEnum', 'src/enum_0.bin', function(r) {
    r._read();
    assert.equal(r._debug.one.enumName, "DebugParamsEnum.Animal");
    assert.equal(r.invokeWithParam._debug.enumeratedOne.enumName, "DebugParamsEnum.Animal");
});
