proc customFx*(src: seq[byte], key: byte): seq[byte] =
  var src: seq[byte] = src
  result = newSeq[byte](src.len + 2)
  result[0] = '_'.byte
  result[^1] = '_'.byte
  copyMem(addr(result[1]), addr(src[0]), src.len)
