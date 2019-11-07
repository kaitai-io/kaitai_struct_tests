import options

proc `==`*[T: SomeInteger](x: seq[T]; y: seq[int]): bool =
  result = true
  for i in 0 ..< x.len:
    if int(x[i]) != y[i]:
      return false
proc `==`*[T](x: Option[T], y: T): bool =
  get(x) == y
converter toByte*(x: int): byte =
  if x < 0: byte(256 + x) else: byte(x)