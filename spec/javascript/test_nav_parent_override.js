var assert = require('assert');
var testHelper = require('testHelper');

testHelper('NavParentOverride', 'src/nav_parent_codes.bin', function(r) {
  assert.equal(r.childSize, 3);
  assert.equal(KaitaiStream.bytesToStr(r.child1.data, "UTF-8"), "I12");
  assert.equal(KaitaiStream.bytesToStr(r.mediator2.child2.data, "UTF-8"), "3Bb");
});
