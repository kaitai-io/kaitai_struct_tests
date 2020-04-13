import sequtils

proc toByte*(n: SomeInteger): byte =
  if n > 255 or n < -255:
    echo "out of bound"
    quit QuitFailure
  elif n < 0:
    byte(255 + n + 1)
  else:
    byte(n)

converter toByteArray*[T: SomeInteger](arr: seq[T]): seq[byte] =
  arr.mapIt(it.toByte)

proc toString*(bytes: seq[byte]): string =
  result = newStringOfCap(bytes.len)
  for b in bytes:
    result.add(b.char)

proc `==`*(actual: seq[byte], expected: string): bool =
  if actual.len != expected.len:
    return false
  for i in 0 ..< actual.len:
    if actual[i] != expected[i].byte:
      return false
  return true

proc `-`*(n: uint64): int64 = -int64(n-1) - 1
