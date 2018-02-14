// This is a generated file! Please edit source .ksy file and use kaitai-struct-compiler to rebuild

(function (root, factory) {
  if (typeof define === 'function' && define.amd) {
    define(['kaitai-struct/KaitaiStream'], factory);
  } else if (typeof module === 'object' && module.exports) {
    module.exports = factory(require('kaitai-struct/KaitaiStream'));
  } else {
    root.MyCustomFx = factory(root.KaitaiStream);
  }
}(this, function (KaitaiStream) {
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
return MyCustomFx;
}));
