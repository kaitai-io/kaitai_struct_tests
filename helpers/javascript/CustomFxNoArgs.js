// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

(function (root, factory) {
  if (typeof define === 'function' && define.amd) {
    define(['kaitai-struct/KaitaiStream'], factory);
  } else if (typeof module === 'object' && module.exports) {
    module.exports = factory(require('kaitai-struct/KaitaiStream'));
  } else {
    root.CustomFxNoArgs = factory(root.KaitaiStream);
  }
}(this, function (KaitaiStream) {
var CustomFxNoArgs = (function() {
  function CustomFxNoArgs() {
  }
  CustomFxNoArgs.prototype.decode = function(src) {
    var len = src.length;
    var dest = new Uint8Array(len + 2);
    for (var i = 0; i < len; i++) {
      dest[i + 1] = src[i];
    }
    dest[0] = 95;
    dest[len + 1] = 95;
    return dest;
  }

  return CustomFxNoArgs;
})();
return CustomFxNoArgs;
}));
