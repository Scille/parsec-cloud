from base64 import encodebytes


def b64(raw):
    return encodebytes(raw).decode()
