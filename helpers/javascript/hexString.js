function hexString(buf) {
    var r = '';
    var len = buf.length;
    for (var i = 0; i < len; i++) {
        var b = buf[i];
        if (i > 0)
            r += ' ';
        if (b < 0x10)
            r += '0';
        r += b.toString(16);
    }
    return r;
}

module.exports = hexString;
