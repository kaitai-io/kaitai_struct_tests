var assert = require('assert');
var testHelper = require('testHelper');

testHelper('NavParentSwitch', 'src/nav_parent_switch.bin', function(r) {
  assert.equal(r.category, 1);
  assert.equal(r.content.foo, 0x42);
  assert.equal(r.content.subelement.bar, 0xff);
});
