meta:
  id: nav_parent_test
  endian: le
seq:
  - id: qty_entries
    type: u4
  - id: filename_len
    type: u4
  - id: index
    type: index_obj
types:
  index_obj:
    seq:
      - id: magic
        size: 4
      - id: entries
        type: entry
        repeat: expr
        repeat-expr: _parent.qty_entries
  entry:
    seq:
      - id: filename
        type: str
        size: _parent._parent.filename_len
        encoding: UTF-8
