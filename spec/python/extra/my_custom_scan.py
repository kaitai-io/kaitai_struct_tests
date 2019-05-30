class MyCustomScan(object):
    def __init__(self, io, byteToScan):
        self._io = io
        self.byteToScan = byteToScan

    def scan(self):
        while True:
          c = self._io.read_u1()
          if c == self.byteToScan:
            return