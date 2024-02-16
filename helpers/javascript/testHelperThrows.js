function testHelperThrows(className, fileName, excClass) {
  var fs = require("fs");
  var assert = require('assert');
  var KaitaiStream = require("kaitai-struct/KaitaiStream");

  describe(className, function() {
    it('parses test properly', function(done) {
      assert.ok(excClass);
      var parser = require(className)[className];
      fs.readFile(fileName, function(err, buf) {
        var st = new KaitaiStream(buf);
        assert.throws(
          function() {
            new parser(st);
          },
          excClass
        );
        done();
      });
    });
  });
}

module.exports = testHelperThrows;
