'use strict';

(function (root, factory) {
  if (typeof define === 'function' && define.amd) {
    define(['exports', 'kaitai-struct/KaitaiStream'], factory);
  } else if (typeof exports === 'object' && exports !== null && typeof exports.nodeType !== 'number') {
    factory(exports, require('kaitai-struct/KaitaiStream'));
  } else {
    factory(root.MyCustomFx || (root.MyCustomFx = {}), root.KaitaiStream);
  }
})(typeof self !== 'undefined' ? self : this, function (MyCustomFx_, KaitaiStream) {
var MyCustomFx = (function() {
  function MyCustomFx(key, flag, someBytes) {
    this.key = flag ? key : -key;
  }
  MyCustomFx.prototype.decode = function(src) {
    var len = src.length;
    var dest = new Uint8Array(len);
    for (var i = 0; i < len; i++) {
      dest[i] = src[i] + this.key;
    }
    return dest;
  }

  return MyCustomFx;
})();
MyCustomFx_.MyCustomFx = MyCustomFx;
});
