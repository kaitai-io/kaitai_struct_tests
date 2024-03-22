class CustomFxNoArgs(object):
    def __init__(self):
        pass

    def decode(self, data):
        return b"_" + data + b"_"

    def encode(self, data):
        assert len(data) >= 2 and data[:1] == b"_" and data[-1:] == b"_", \
            "CustomFxNoArgs can only encode data like '_(.*)_'"

        return data[1:-1]
