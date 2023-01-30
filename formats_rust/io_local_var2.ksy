meta:
  id: io_local_var2
seq:
  - id: some_header
    size: 10
  - id: files
    size: 80
    type: file_entry
types:
  file_entry:
    seq:
      - id: ofs_body
        type: u1
      - id: len_body
        type: u1
    instances:
      body:
        io: _root._io
        pos: ofs_body
        size: len_body