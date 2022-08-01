class CustomFxNoArgs(object):
    def __init__(self):
        pass

    def decode(self, data):
        return b"_" + data + b"_"
