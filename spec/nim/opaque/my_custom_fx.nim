proc myCustomFx*(src: string, key: byte, flag: bool, someBytes: string): string =
  result = newString(src.len)
  let key = if flag: key else: byte(-int8(key-1) - 1)
  for i in 0 ..< src.len:
    result[i] = char(byte(src[i]) + key)
