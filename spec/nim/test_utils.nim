proc toByte*(n: SomeInteger): byte =
  if n > 255 or n < -255:
    echo "out of bound"
    quit QuitFailure
  elif n < 0:
    byte(255 + n + 1)
  else:
    byte(n)

proc toString*(bytes: seq[byte]): string =
  result = newStringOfCap(bytes.len)
  for b in bytes:
    result.add(b.char)

proc toString*(nums: seq[int8]): string =
  result = newStringOfCap(len(nums))
  for n in nums:
    let b = if n < 0: 255 + n + 1
            else: n
    add(result, char(b))
