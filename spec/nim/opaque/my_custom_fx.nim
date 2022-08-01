proc myCustomFx*(src: seq[byte], key: byte, flag: bool, someBytes: seq[SomeInteger]): seq[byte] =
  result = newSeq[byte](src.len)
  let key = if flag: key else: byte(-int8(key-1) - 1)
  for i in 0 ..< src.len:
    result[i] = src[i] + key
