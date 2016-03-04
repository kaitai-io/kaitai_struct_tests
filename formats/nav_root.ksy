meta:
  id: nav_root
  endian: le
seq:
  - id: header
    type: header
  - id: index
    type: index
types:
  header:
    seq:
      - id: qty_entries
        type: u4
      - id: filename_len
        type: u4
  index:
    seq:
      - id: magic
        size: 4
      - id: entries
        type: entry
        repeat: expr
        repeat-expr: _root.header.qty_entries
  entry:
    seq:
      - id: filename
        type: str
        size: _root.header.filename_len
        encoding: UTF-8
