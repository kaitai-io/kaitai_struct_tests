import io
import os
import shutil
import tempfile
from abc import abstractmethod, ABC
from contextlib import contextmanager
from enum import Enum

from kaitaistruct import KaitaiStream


class FileOpenMode(Enum):
    READ_ONLY = 1
    WRITE_ONLY = 2
    READ_WRITE = 3

    @property
    def readable(self):
        return self == FileOpenMode.READ_ONLY or self == FileOpenMode.READ_WRITE

    @property
    def writable(self):
        return self == FileOpenMode.WRITE_ONLY or self == FileOpenMode.READ_WRITE

    @property
    def mode(self):
        if self == FileOpenMode.READ_ONLY:
            return 'rb'
        elif self == FileOpenMode.WRITE_ONLY:
            return 'wb'
        elif self == FileOpenMode.READ_WRITE:
            return 'wb+'

    @property
    def ident(self):
        if self == FileOpenMode.READ_ONLY:
            return 'rdonly'
        elif self == FileOpenMode.WRITE_ONLY:
            return 'wronly'
        elif self == FileOpenMode.READ_WRITE:
            return 'rdwr'


class FileOpenConfig(object):
    def __init__(self, buffering, use_builtin_open):
        self.buffering = buffering
        self.use_builtin_open = use_builtin_open


class RegularFileOpener(object):
    @staticmethod
    def open_file_object(f, size):
        try:
            if size is not None:
                f.truncate(size)

            return KaitaiStream(f)
        except Exception:
            f.close()
            raise

    @classmethod
    def open_path(cls, file_path, open_mode, open_conf, size=None):
        if open_conf.use_builtin_open:
            f = open(file_path, open_mode.mode, buffering=open_conf.buffering)
        else:
            f = io.open(file_path, open_mode.mode, buffering=open_conf.buffering)

        return cls.open_file_object(f, size)

    @classmethod
    def open_fd(cls, fd, open_mode, open_conf, size=None):
        try:
            if open_conf.use_builtin_open:
                # must use positional arguments (otherwise we get "TypeError: fdopen() takes no
                # keyword arguments" in Python 2)
                f = os.fdopen(fd, open_mode.mode, open_conf.buffering)
            else:
                f = io.open(fd, open_mode.mode, buffering=open_conf.buffering)
        except Exception:
            os.close(fd)
            raise

        return cls.open_file_object(f, size)


class AbstractStream(ABC):
    @abstractmethod
    def open(self):
        pass

    @abstractmethod
    def open_as_read_only(self):
        pass


class TemporaryFile(AbstractStream):
    def __init__(self, open_mode, open_conf, size):
        self.tmp_fd, self.tmp_path = tempfile.mkstemp()

        self.open_mode = open_mode
        self.open_conf = open_conf
        self.size = size

    def __enter__(self):
        return self

    def __exit__(self, *args):
        self.destroy()

    def open(self):
        return RegularFileOpener.open_fd(self.tmp_fd, self.open_mode, self.open_conf, self.size)

    def open_as_read_only(self):
        return RegularFileOpener.open_path(self.tmp_path, FileOpenMode.READ_ONLY, self.open_conf)

    def destroy(self):
        os.remove(self.tmp_path)
        self.tmp_path = None


class Pipe(AbstractStream):
    def __init__(self, open_conf):
        self.r_fd = None

        self.open_conf = open_conf

    def __enter__(self):
        return self

    def __exit__(self, *args):
        self.destroy()

    def init_from_path(self, file_path):
        self.r_fd, w_fd = os.pipe()
        with io.open(w_fd, 'wb') as dst_f, io.open(file_path, 'rb') as src_f:
            shutil.copyfileobj(src_f, dst_f)

    def open(self):
        """
        At the time of writing, the KaitaiStream class doesn't support writing to non-seekable
        streams (like pipes), but this method is kept in case it's supported in some form in the
        future.
        """
        self.r_fd, w_fd = os.pipe()
        return RegularFileOpener.open_fd(w_fd, FileOpenMode.WRITE_ONLY, self.open_conf)

    def open_as_read_only(self):
        if self.r_fd is None:
            raise ValueError(
                "{}() or {}() must be called first"
                .format(self.open.__name__, self.init_from_path.__name__)
            )

        r_fd = self.r_fd
        self.r_fd = None
        return RegularFileOpener.open_fd(r_fd, FileOpenMode.READ_ONLY, self.open_conf)

    def destroy(self):
        if self.r_fd is not None:
            os.close(self.r_fd)
            self.r_fd = None


class InMemoryStream(AbstractStream):
    def __init__(self, f):
        self.ks_io = KaitaiStream(f)

    def __enter__(self):
        return self

    def __exit__(self, *args):
        self.destroy()

    @classmethod
    def from_size(cls, size):
        f = io.BytesIO()
        if size > 0:
            f.seek(size - 1, io.SEEK_SET)
            f.write(b'\x00')
            f.seek(0, io.SEEK_SET)

        return cls(f)

    @classmethod
    def from_path(cls, file_path):
        f = io.BytesIO()
        with io.open(file_path, 'rb') as src_f:
            shutil.copyfileobj(src_f, f)
        f.seek(0, io.SEEK_SET)

        return cls(f)

    @contextmanager
    def _open_contextmanager(self):
        try:
            yield self.ks_io
        finally:
            self.ks_io.seek(0)

    def open(self):
        return self._open_contextmanager()

    def open_as_read_only(self):
        return self.open()

    def destroy(self):
        self.ks_io.close()
        self.ks_io = None
