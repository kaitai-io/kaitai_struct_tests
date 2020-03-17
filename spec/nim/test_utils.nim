proc toByte*(n: SomeInteger): byte =
  if n > 255 or n < -255:
    echo "out of bound"
    quit QuitFailure
  elif n < 0:
    byte(255 + n + 1)
  else:
    byte(n)
