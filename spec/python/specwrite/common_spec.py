import unittest

from kaitaistruct import KaitaiStream, KaitaiStruct, PY2

from decorators import stream_param_tests, write_stream_param


# A little hack from https://stackoverflow.com/a/25695512 to trick 'unittest'
# into thinking that CommonSpec.Base is not a test by itself.
class CommonSpec(object):
    @stream_param_tests
    class Base(unittest.TestCase):
        def __init__(self, *args, **kwargs):
            super(CommonSpec.Base, self).__init__(*args, **kwargs)
            self.maxDiff = None
            self.skip_roundtrip_msg_reason = None

        @write_stream_param
        def test_read_write_roundtrip(self, stream_builder):
            if self.skip_roundtrip_msg_reason is not None:
                self.skipTest(self.skip_roundtrip_msg_reason)

            with self.struct_class.from_file(self.src_filename) as orig_ks:
                orig_ks._read()
                orig_dump = CommonSpec.Base.dump_struct(orig_ks)
                orig_io_size = orig_ks._io.size()

            with stream_builder(orig_io_size) as obj:
                with obj.open() as ks_io:
                    self.assertEqual(ks_io.size(), orig_io_size)
                    orig_ks._write(ks_io)

                with obj.open_as_read_only() as ks_io:
                    new_ks = self.struct_class(ks_io)
                    new_ks._read()

                    new_dump = CommonSpec.Base.dump_struct(new_ks)

            self.assertEqual(orig_dump, new_dump)

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

                    # call all _check*() methods
                    if prop_name.startswith('_check'):
                        prop_value()
                        continue

                    if callable(prop_value):
                        continue

                    if (
                        prop_name == '_io' or \
                        prop_name.startswith('_raw_')
                    ):
                        continue

                    if (
                        prop_name.startswith('_should_write') or \
                        prop_name.endswith('__outer_size') or \
                        prop_name.endswith('__inner_size') or \
                        prop_name.endswith('__to_write')
                    ):
                        continue

                    dump[prop_name] = CommonSpec.Base.dump_struct_value(
                        prop_value, parent_structs, recursion_depth_limit - 1,
                        current_path + ('' if current_path == '/' else '/') + prop_name
                    )

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

            if PY2 and isinstance(value, bytes):
                value = bytearray(value)

            if isinstance(value, (bytes, bytearray)):
                # https://stackoverflow.com/a/19210468
                value = ' '.join('%02x' % b for b in value)

            return value
