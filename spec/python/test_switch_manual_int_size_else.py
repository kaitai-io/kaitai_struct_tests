import unittest

from switch_manual_int_size_else import SwitchManualIntSizeElse

class TestSwitchManualIntSizeElse(unittest.TestCase):
    def test_switch_manual_int_size_else(self):
        r = SwitchManualIntSizeElse.from_file("src/switch_tlv.bin")

        self.assertEquals(len(r.chunks), 4)

        self.assertEquals(r.chunks[0].code, 0x11)
        meta = r.chunks[0].body
        self.assertEquals(meta.title, "Stuff")
        self.assertEquals(meta.author, "Me")

        self.assertEquals(r.chunks[1].code, 0x22)
        self.assertEquals(r.chunks[1].body.entries, ["AAAA", "BBBB", "CCCC"])

        self.assertEquals(r.chunks[2].code, 0x33)
        self.assertEquals(r.chunks[2].body.rest, bytearray([0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80]))

        self.assertEquals(r.chunks[3].code, 0xff)
        self.assertEquals(r.chunks[3].body.rest, "")
