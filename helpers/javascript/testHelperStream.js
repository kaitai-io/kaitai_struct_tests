function testHelperStream(className, fileName, testFunc) {
  var fs = require("fs");
  var KaitaiStream = global.KaitaiStream = require("kaitai-struct/KaitaiStream");
  var parser = require(className);

  describe(className, function() {
    it('parses test properly', function(done) {
      fs.readFile(fileName, function(err, buf) {
        var st = new KaitaiStream(buf);
        testFunc(st, parser);
        done();
      });
    });
  });
}

module.exports = testHelperStream;
