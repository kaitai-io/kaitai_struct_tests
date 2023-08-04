import os.path
import unittest

from helpers import FileOpenConfig, FileOpenMode, InMemoryStream, Pipe, RegularFileOpener, \
    TemporaryFile

TEST_FILE_PATH = os.path.join(
    os.path.dirname(os.path.realpath(__file__)),
    '../../../src/nav_parent_switch.bin'
)


class TestHelpers(unittest.TestCase):
    def test_RegularFileOpener_open_path(self):
        open_mode = FileOpenMode.READ_ONLY
        open_conf = FileOpenConfig(-1, False)
        with RegularFileOpener.open_path(TEST_FILE_PATH, open_mode, open_conf) as ks_io:
            self.assertEqual(ks_io.read_bytes_full(), b'\x01\x42\xff')

    def test_TemporaryFile(self):
        open_conf = FileOpenConfig(-1, False)
        with TemporaryFile(FileOpenMode.WRITE_ONLY, open_conf, 6) as tf:
            with tf.open() as ks_io:
                self.assertEqual(ks_io.size(), 6)
                ks_io.write_bytes(b'Hello\0')

            file_path = tf.tmp_path
            self.assertTrue(os.path.isfile(file_path))

            with tf.open_as_read_only() as ks_io:
                self.assertEqual(ks_io.read_bytes_full(), b'Hello\0')

        self.assertFalse(os.path.lexists(file_path))

    def test_TemporaryFile_no_usage(self):
        """ Ensure that we don't get any errors (that could be potentially raised from __exit__()
        which is called automatically) if we create a TemporaryFile but do nothing with it.
        """
        open_conf = FileOpenConfig(-1, False)
        with TemporaryFile(FileOpenMode.WRITE_ONLY, open_conf, 6) as _tf:
            pass

    def test_TemporaryFile_uncaught_exception_before_open(self):
        """ Ensure that we don't get any errors (that could be potentially raised from __exit__()
        which is called automatically) if we create a TemporaryFile but then there's an uncaught
        exception before we call open().
        """
        open_conf = FileOpenConfig(-1, False)
        with self.assertRaises(ValueError) as cm:
            with TemporaryFile(FileOpenMode.WRITE_ONLY, open_conf, 6) as _tf:
                raise ValueError("fooBarQux happened")
                # here we would call open() etc.

        self.assertEqual(str(cm.exception), "fooBarQux happened")

    def test_TemporaryFile_open_as_read_only_too_soon(self):
        open_conf = FileOpenConfig(-1, False)
        with TemporaryFile(FileOpenMode.WRITE_ONLY, open_conf, 6) as tf:
            with self.assertRaises(ValueError) as cm:
                with tf.open_as_read_only() as _ks_io:
                    pass
            self.assertEqual(str(cm.exception), "open() must be called first")

    def test_InMemoryStream_from_size(self):
        with InMemoryStream.from_size(6) as ims:
            with ims.open() as ks_io:
                self.assertEqual(ks_io.size(), 6)
                ks_io.write_bytes(b'Hello\0')
            with ims.open_as_read_only() as ks_io:
                self.assertEqual(ks_io.read_bytes_full(), b'Hello\0')

    def test_InMemoryStream_from_path(self):
        with InMemoryStream.from_path(TEST_FILE_PATH) as ims:
            with ims.open() as ks_io:
                self.assertEqual(ks_io.read_bytes_full(), b'\x01\x42\xff')

    def test_Pipe(self):
        open_conf = FileOpenConfig(0, False)
        with Pipe(open_conf) as p:
            p.init_from_path(TEST_FILE_PATH)
            with p.open_as_read_only() as ks_io:
                self.assertEqual(ks_io.read_u1(), 0x01)
                self.assertEqual(ks_io.read_bytes_full(), b'\x42\xff')

    def test_Pipe_no_usage(self):
        open_conf = FileOpenConfig(0, False)
        with Pipe(open_conf) as _p:
            pass

    def test_Pipe_open_as_read_only_too_soon(self):
        open_conf = FileOpenConfig(0, False)
        with Pipe(open_conf) as p:
            with self.assertRaises(ValueError) as cm:
                with p.open_as_read_only() as _ks_io:
                    pass
            self.assertEqual(str(cm.exception), "init_from_path() or open() must be called first")


if __name__ == '__main__':
    unittest.main()
