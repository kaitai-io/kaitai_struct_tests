function testHelper(className, fileName, testFunc) {
  var fs = require("fs");
//  var DataStream = require("DataStream");
  var KaitaiStream = require("KaitaiStream");
  var parser = require(className);

  describe(className, function() {
    it('parses test properly', function(done) {
      fs.readFile(fileName, function(err, buf) {
//        var st = new DataStream(buf);
        var st = new KaitaiStream(buf);
        var r = new parser(st);
        testFunc(r, parser);
        done();
      });
    });
  });
}

module.exports = testHelper;
