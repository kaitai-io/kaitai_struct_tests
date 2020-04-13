proc customFx*(src: string, key: byte): string =
  var src: string = src
  result = newString(src.len + 2)
  result[0] = '_'
  result[^1] = '_'
  copyMem(addr(result[1]), addr(src[0]), src.len)
