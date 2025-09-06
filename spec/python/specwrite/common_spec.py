import unittest
import io
from kaitaistruct import KaitaiStream, KaitaiStruct, PY2

# A little hack from https://stackoverflow.com/a/25695512 to trick 'unittest'
# into thinking that CommonSpec.Base is not a test by itself.
class CommonSpec:

    class Base(unittest.TestCase):
        def __init__(self, *args, **kwargs):
            super(CommonSpec.Base, self).__init__(*args, **kwargs)
            self.maxDiff = None

        def test_read_write_roundtrip(self):
            orig_f = io.open(self.src_filename, 'rb')

            try:
                orig_ks = self.struct_class.from_io(orig_f)
                orig_ks._read()

                orig_dump = CommonSpec.Base.dump_struct(orig_ks)

                orig_io_size = orig_ks._io.size()
            finally:
                orig_f.close()

            with KaitaiStream(io.BytesIO(bytearray(orig_io_size))) as new_io:
                orig_ks._write(new_io)
                new_io.seek(0)

                new_ks = self.struct_class(new_io)
                new_ks._read()

                new_dump = CommonSpec.Base.dump_struct(new_ks)

            self.assertEqual(orig_dump, new_dump)

        def assert_equal_to_full_file(self, struct_obj, file_name):
            with io.open(file_name, 'rb') as f:
                expected_bytes = f.read()
            struct_obj._check()
            with KaitaiStream(io.BytesIO(bytearray(len(expected_bytes)))) as out_io:
                struct_obj._write(out_io)
                self.assert_byte_array_equal(out_io.to_byte_array(), expected_bytes)

        def assert_byte_array_equal(self, actual, expected):
            self.assertEqual(
                CommonSpec.Base.byte_array_to_hex(actual),
                CommonSpec.Base.byte_array_to_hex(expected),
            )

        @staticmethod
        def dump_struct(obj):
            return CommonSpec.Base.dump_struct_value(obj, [], 50, '/')

        @staticmethod
        def dump_struct_value(value, parent_structs, recursion_depth_limit, current_path):
            if recursion_depth_limit < 0:
                raise RuntimeError("recursion depth limit reached")

            if isinstance(value, KaitaiStruct):
                dump = {}
                for obj, ref in parent_structs:
                    if value is obj:
                        dump['$ref'] = ref
                        return dump

                parent_structs.append((value, current_path))

                for prop_name in dir(value):
                    if prop_name.startswith('__'):
                        continue
                    prop_value = getattr(value, prop_name)

                    if isinstance(prop_value, type):
                        continue

                    if callable(prop_value):
                        continue

                    if (
                        prop_name == '_io' or \
                        prop_name == '_debug' or \
                        prop_name == '_dirty' or \
                        prop_name == 'SEQ_FIELDS' or \
                        prop_name.startswith('_raw_') or \
                        prop_name.startswith('_m_')
                    ):
                        continue

                    if (
                        prop_name.startswith('_should_write') or \
                        prop_name.endswith('__outer_size') or \
                        prop_name.endswith('__inner_size') or \
                        prop_name.endswith('__enabled')
                    ):
                        continue

                    dump[prop_name] = CommonSpec.Base.dump_struct_value(
                        prop_value, parent_structs, recursion_depth_limit - 1,
                        current_path + ('' if current_path == '/' else '/') + prop_name
                    )

                # We call the `_check()` method after dumping the object. This is
                # necessary: if we did this before the object is fully dumped, some parse
                # instances might not have been invoked yet (meaning that their `_m_*`
                # attributes wouldn't be defined, so it would fail).
                value._check()

                assert parent_structs.pop()[0] is value

                return dump

            if isinstance(value, list):
                dump = [
                    CommonSpec.Base.dump_struct_value(
                        item, parent_structs, recursion_depth_limit - 1,
                        current_path + ('' if current_path == '/' else '/') + str(i)
                    )
                    for i, item in enumerate(value)
                ]
                return dump

            if isinstance(value, KaitaiStream):
                value = value.to_byte_array()

            if isinstance(value, (bytes, bytearray)):
                # https://stackoverflow.com/a/19210468
                value = CommonSpec.Base.byte_array_to_hex(value)

            return value

        @staticmethod
        def byte_array_to_hex(arr):
            if PY2 and isinstance(arr, bytes):
                arr = bytearray(arr)

            return ' '.join('%02x' % b for b in arr)

# In Python 2, the method is called `assertRaisesRegexp`, see
# https://github.com/kaitai-io/kaitai_struct_tests/pull/115#issue-2231549324
#
# Once we end support for Python 2, we can remove this code
# (`assertRaisesRegex` is available since Python 3.2).
if not hasattr(CommonSpec.Base, 'assertRaisesRegex'):
    CommonSpec.Base.assertRaisesRegex = CommonSpec.Base.assertRaisesRegexp
