import io
import itertools
from functools import wraps
from types import FunctionType

from kaitaistruct import PY2

from helpers import FileOpenMode, FileOpenConfig, TemporaryFile, InMemoryStream


def stream_param_tests(cls):
    # From https://github.com/adamchainz/unittest-parametrize/blob/58cf079/src/unittest_parametrize/__init__.py#L24-L28
    for name, func in list(cls.__dict__.items()):
        if not isinstance(func, FunctionType):
            continue
        if not getattr(func, '_parametrized_by_write_stream', False):
            continue

        delattr(cls, name)

        def test_builder(stream_builder, func=func):
            @wraps(func)
            def wrapper(self):
                return func(self, stream_builder)
            return wrapper

        for test_method in _generate_write_subtests(name, test_builder):
            setattr(cls, test_method.__name__, test_method)

    return cls


def write_stream_param(func):
    func._parametrized_by_write_stream = True
    return func


def _generate_write_subtests(test_basename, test_builder):
    use_builtin_open_options = (False,)
    # in Python 3, built-in open() is an alias of io.open(); in Python 2, open() is
    # a different function than io.open()
    builtin_open_is_not_io_open = open is not io.open
    assert builtin_open_is_not_io_open == PY2, (
        "expected the built-in open() to be different from io.open() only in Python 2, "
        "but builtin_open_is_not_io_open={} and PY2={}"
        .format(builtin_open_is_not_io_open, PY2)
    )

    if builtin_open_is_not_io_open:
        use_builtin_open_options += (True,)
    open_modes = [mode for mode in FileOpenMode if mode.writable]
    buffering_options = (-1, 0)

    generated_test_methods = []

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

        test_method = test_builder(stream_builder)
        test_method.__name__ = \
            '{}__TemporaryFile_{}open_{}{}'.format(
                test_basename,
                'builtin_' if use_builtin_open else 'io_',
                open_mode.ident,
                '_nobuf' if buffering == 0 else '',
            )
        generated_test_methods.append(test_method)

    def stream_builder(orig_io_size):
        return InMemoryStream.from_size(orig_io_size)

    test_method = test_builder(stream_builder)
    test_method.__name__ = '{}__InMemoryStream'.format(test_basename)
    generated_test_methods.append(test_method)

    return generated_test_methods
