import unittest

from switch_manual_int_size import SwitchManualIntSize

class TestSwitchManualIntSize(unittest.TestCase):
    def test_switch_manual_int_size(self):
        r = SwitchManualIntSize.from_file("src/switch_tlv.bin")

        self.assertEqual(len(r.chunks), 4)

        self.assertEqual(r.chunks[0].code, 0x11)
        meta = r.chunks[0].body
        self.assertEqual(meta.title, "Stuff")
        self.assertEqual(meta.author, "Me")

        self.assertEqual(r.chunks[1].code, 0x22)
        self.assertEqual(r.chunks[1].body.entries, ["AAAA", "BBBB", "CCCC"])

        self.assertEqual(r.chunks[2].code, 0x33)
        self.assertEqual(r.chunks[2].body, bytearray([0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80]))

        self.assertEqual(r.chunks[3].code, 0xff)
        self.assertEqual(r.chunks[3].body, b"")
