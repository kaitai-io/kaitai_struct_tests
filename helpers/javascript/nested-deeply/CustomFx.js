'use strict';

(function (root, factory) {
  if (typeof define === 'function' && define.amd) {
    define(['exports', 'kaitai-struct/KaitaiStream'], factory);
  } else if (typeof exports === 'object' && exports !== null && typeof exports.nodeType !== 'number') {
    factory(exports, require('kaitai-struct/KaitaiStream'));
  } else {
    factory(root.CustomFx || (root.CustomFx = {}), root.KaitaiStream);
  }
})(typeof self !== 'undefined' ? self : this, function (CustomFx_, KaitaiStream) {
var CustomFx = (function() {
  function CustomFx(key) {
    this.key = key;
  }
  CustomFx.prototype.decode = function(src) {
    var len = src.length;
    var dest = new Uint8Array(len + 2);
    for (var i = 0; i < len; i++) {
      dest[i + 1] = src[i];
    }
    dest[0] = 95;
    dest[len + 1] = 95;
    return dest;
  }

  return CustomFx;
})();
CustomFx_.CustomFx = CustomFx;
});
