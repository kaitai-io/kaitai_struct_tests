var assert = require('assert');
var testHelper = require('testHelper');

testHelper('NavParent3', 'src/nav_parent2.bin', function(r) {
  assert.equal(r.ofsTags, 8);
  assert.equal(r.numTags, 2);

  assert.equal(r.tags[0].name, 'RAHC');
  assert.equal(r.tags[0].ofs, 0x20);
  assert.equal(r.tags[0].numItems, 3);
  assert.equal(r.tags[0].tagContent.content, 'foo');

  assert.equal(r.tags[1].name, 'RAHC');
  assert.equal(r.tags[1].ofs, 0x23);
  assert.equal(r.tags[1].numItems, 6);
  assert.equal(r.tags[1].tagContent.content, 'barbaz');
});
