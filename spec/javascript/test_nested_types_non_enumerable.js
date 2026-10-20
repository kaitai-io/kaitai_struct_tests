var assert = require('assert');
var testHelper = require('testHelper');

testHelper('NestedTypes', 'src/fixed_struct.bin', function(r, NestedTypes_) {
  // First verify the data was parsed correctly
  assert.strictEqual(r.one.typedAtRoot.valueB, 80);
  assert.strictEqual(r.one.typedHere.valueC, 65);
  assert.strictEqual(r.two.valueB, 67);
  
  // Test that internal properties exist but are not enumerable
  // Check root object
  assert.strictEqual(typeof r._io, 'object', '_io should exist');
  assert.strictEqual(typeof r._parent, 'undefined', '_parent should be undefined for root');
  assert.strictEqual(typeof r._root, 'object', '_root should exist');
  assert.strictEqual(r._root, r, '_root should be self for root object');
  
  // Check that internal properties are not enumerable on root object
  var rootKeys = Object.keys(r);
  assert.strictEqual(rootKeys.indexOf('_io'), -1, '_io should not be enumerable');
  assert.strictEqual(rootKeys.indexOf('_parent'), -1, '_parent should not be enumerable');
  assert.strictEqual(rootKeys.indexOf('_root'), -1, '_root should not be enumerable');
  
  // Check enumerable properties on root object
  assert.notStrictEqual(rootKeys.indexOf('one'), -1, 'one should be enumerable');
  assert.notStrictEqual(rootKeys.indexOf('two'), -1, 'two should be enumerable');
  
  // Check nested object (r.one)
  assert.strictEqual(typeof r.one._io, 'object', 'one._io should exist');
  assert.strictEqual(r.one._parent, r, 'one._parent should be root object');
  assert.strictEqual(r.one._root, r, 'one._root should be root object');
  
  // Check that internal properties are not enumerable on nested object
  var oneKeys = Object.keys(r.one);
  assert.strictEqual(oneKeys.indexOf('_io'), -1, 'one._io should not be enumerable');
  assert.strictEqual(oneKeys.indexOf('_parent'), -1, 'one._parent should not be enumerable');
  assert.strictEqual(oneKeys.indexOf('_root'), -1, 'one._root should not be enumerable');
  
  // Check enumerable properties on nested object
  assert.notStrictEqual(oneKeys.indexOf('typedAtRoot'), -1, 'typedAtRoot should be enumerable');
  assert.notStrictEqual(oneKeys.indexOf('typedHere'), -1, 'typedHere should be enumerable');
  
  // Test JSON.stringify excludes internal properties
  var json = JSON.stringify(r);
  var parsed = JSON.parse(json);
  
  assert.strictEqual(parsed.one.typedAtRoot.valueB, 80, 'JSON should include one.typedAtRoot.valueB');
  assert.strictEqual(parsed.one.typedHere.valueC, 65, 'JSON should include one.typedHere.valueC');
  assert.strictEqual(parsed.two.valueB, 67, 'JSON should include two.valueB');
  
  assert.strictEqual(parsed._io, undefined, 'JSON should not include _io');
  assert.strictEqual(parsed._parent, undefined, 'JSON should not include _parent');
  assert.strictEqual(parsed._root, undefined, 'JSON should not include _root');
  assert.strictEqual(parsed.one._io, undefined, 'JSON should not include one._io');
  assert.strictEqual(parsed.one._parent, undefined, 'JSON should not include one._parent');
  assert.strictEqual(parsed.one._root, undefined, 'JSON should not include one._root');
  
  // Test for...in loop excludes internal properties
  var foundInForIn = [];
  for (var key in r) {
    foundInForIn.push(key);
  }
  
  assert.strictEqual(foundInForIn.indexOf('_io'), -1, '_io should not appear in for...in');
  assert.strictEqual(foundInForIn.indexOf('_parent'), -1, '_parent should not appear in for...in');
  assert.strictEqual(foundInForIn.indexOf('_root'), -1, '_root should not appear in for...in');
});