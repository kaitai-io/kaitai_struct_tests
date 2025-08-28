import sequtils
from kaitai_struct_nim_runtime import `-`
export kaitai_struct_nim_runtime.`-`

proc `-`*(n: uint64): int64 = -int64(n - 1) - 1

## Loose equality

proc `==`*[S: SomeSignedInt, U: SomeUnsignedInt](x: S, y: U): bool =
  x == S(y)

proc `==`*[S: SomeSignedInt, U: SomeUnsignedInt](x: U, y: S): bool =
  S(x) == y
