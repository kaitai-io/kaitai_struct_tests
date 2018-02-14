import unittest

from default_endian_expr_inherited import DefaultEndianExprInherited

class TestDefaultEndianExprInherited(unittest.TestCase):
    def test_default_endian_expr_inherited(self):
        r = DefaultEndianExprInherited.from_file("src/endian_expr.bin")

        self.assertEqual(r.docs[0].indicator, b"II")
        self.assertEqual(r.docs[0].main.insides.some_int, 0x42)
        self.assertEqual(r.docs[0].main.insides.more.some_int1, 0x4200)
        self.assertEqual(r.docs[0].main.insides.more.some_int2, 0x42)
        self.assertEqual(r.docs[0].main.insides.more.some_inst, 0x42)

        self.assertEqual(r.docs[1].indicator, b"MM")
        self.assertEqual(r.docs[1].main.insides.some_int, 0x42)
        self.assertEqual(r.docs[1].main.insides.more.some_int1, 0x42)
        self.assertEqual(r.docs[1].main.insides.more.some_int2, 0x4200)
        self.assertEqual(r.docs[1].main.insides.more.some_inst, 0x42000000)

        self.assertEqual(r.docs[2].indicator, b"XX")
        self.assertEqual(r.docs[2].main.insides.some_int, 0x42)
        self.assertEqual(r.docs[2].main.insides.more.some_int1, 0x42)
        self.assertEqual(r.docs[2].main.insides.more.some_int2, 0x4200)
        self.assertEqual(r.docs[2].main.insides.more.some_inst, 0x42000000)
