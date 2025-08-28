function testHelper(className, fileName, testFunc) {
  var fs = require("fs");
  var KaitaiStream = global.KaitaiStream = require("kaitai-struct/KaitaiStream");

  describe(className, function() {
    it('parses test properly', function(done) {
      var parser_ = require(className);
      fs.readFile(fileName, function(err, buf) {
        if (err) {
          done(err);
          return;
        }
        var st = new KaitaiStream(buf);
        var r = new parser_[className](st);
        try {
          testFunc(r, parser_);
        } catch (e) {
          done(e);
          return;
        }
        done();
      });
    });
  });
}

module.exports = testHelper;
