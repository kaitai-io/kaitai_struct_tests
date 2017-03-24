meta:
  id:      process_bcd_to_be
  endian:  be

seq:
  - id:       ltr
    size:     4
    process:  bcd_to_str(ltr)
    type:     ltr
  - id:       rtl
    size:     4
    process:  bcd_to_str(rtl)
    type:     rtl
  - id:       decimal
    size:     4
    process:  bcd_to_decimal
    type:     decimal

types:
  ltr:
    seq:
      - id:       value
        size-eos: true
        type:     str
        encoding: ascii
  rtl:
    seq:
      - id:       value
        size-eos: true
        type:     str
        encoding: ascii
  decimal:
    seq:
      - id:       value
        type:     u4
