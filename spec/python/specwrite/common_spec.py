import io
import itertools
import unittest
from kaitaistruct import KaitaiStream, KaitaiStruct, PY2

from helpers import InMemoryStream, TemporaryFile, FileOpenMode, FileOpenConfig


# A little hack from https://stackoverflow.com/a/25695512 to trick 'unittest'
# into thinking that CommonSpec.Base is not a test by itself.
class CommonSpec(object):
    class Base(unittest.TestCase):
        def __init__(self, *args, **kwargs):
            super(CommonSpec.Base, self).__init__(*args, **kwargs)
            self.maxDiff = None
            self.skip_roundtrip_msg_reason = None

        @classmethod
        def generate_test_read_write_roundtrip_variants(cls):
            use_builtin_open_options = (False,)
            # in Python 3, built-in open() is an alias of io.open(); in Python 2, open() is
            # a different function than io.open()
            builtin_open_is_not_io_open = open is not io.open
            if builtin_open_is_not_io_open != PY2:
                raise ValueError(
                    "expected the built-in open() to be different from io.open() only in Python 2, "
                    "but builtin_open_is_not_io_open={} and PY2={}"
                    .format(builtin_open_is_not_io_open, PY2)
                )

            if builtin_open_is_not_io_open:
                use_builtin_open_options += (True,)
            # See https://stackoverflow.com/a/67558256/12940655 for a table of possible modes
            open_modes = (
                FileOpenMode.WRITE_ONLY,
                FileOpenMode.READ_WRITE,
            )
            buffering_options = (-1, 0)

            # NOTE: We're not using unittest.TestCase.subTest
            # (https://docs.python.org/3/library/unittest.html#unittest.TestCase.subTest) for two
            # reasons:
            #   1. it's only available in Python 3, not Python 2
            #   2. the "unittest-xml-reporting" Python package that we use to generate XML test
            #      reports has a limited support for subtests, see
            #      https://github.com/xmlrunner/unittest-xml-reporting/tree/3.2.0#limited-support-for-unittesttestcasesubtest
            for use_builtin_open, open_mode, buffering in itertools.product(
                    use_builtin_open_options, open_modes, buffering_options
            ):
                open_conf = FileOpenConfig(buffering, use_builtin_open)

                def stream_builder(orig_io_size, open_mode=open_mode, open_conf=open_conf):
                    return TemporaryFile(open_mode, open_conf, orig_io_size)

                test_method = cls.create_test_read_write_roundtrip(stream_builder)
                test_method.__name__ = \
                    'test_read_write_roundtrip__TemporaryFile_{}open_{}{}'.format(
                        'builtin_' if use_builtin_open else 'io_',
                        open_mode.ident,
                        '_nobuf' if buffering == 0 else '',
                    )
                setattr(cls, test_method.__name__, test_method)

            def stream_builder(orig_io_size):
                return InMemoryStream.from_size(orig_io_size)

            test_method = cls.create_test_read_write_roundtrip(stream_builder)
            test_method.__name__ = 'test_read_write_roundtrip__InMemoryStream'
            setattr(cls, test_method.__name__, test_method)

        @classmethod
        def create_test_read_write_roundtrip(cls, stream_builder):
            def test_body(self):
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

            return test_body

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


CommonSpec.Base.generate_test_read_write_roundtrip_variants()
