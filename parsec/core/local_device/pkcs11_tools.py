# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import io
from structlog import get_logger

try:
    import pkcs11
    from pkcs11 import Attribute, KeyType, ObjectClass, Mechanism
    from pkcs11.util.rsa import encode_rsa_public_key

    from Crypto import Random
    from Crypto.Cipher import PKCS1_v1_5, AES
    from Crypto.PublicKey import RSA

    PKCS11_AVAILABLE = True

except ImportError:
    PKCS11_AVAILABLE = False


pkcs_module_path = [
    "/usr/lib/opensc-pkcs11.so",
    "/usr/lib/x86_64-linux-gnu/pkcs11/opensc-pkcs11.so",
    "/usr/local/lib/pkcs11/opensc-pkcs11.so",
]

# consts for 4096
ENCRYPTED_READ_BYTES_COUNT = {2048: 256, 4096: 256 * 2}
# consts for 2048
PLAINTEXT_READ_BYTES_COUNT = {2048: 240, 4096: 240 * 2}

LIB = None

logger = get_logger()


class DevicePKCS11Error(Exception):
    pass


class NoKeysFound(DevicePKCS11Error):
    pass


def get_LIB():
    global LIB
    if LIB:
        return LIB
    for path in pkcs_module_path:
        try:
            logger.info("Loading library from " + path)
            LIB = pkcs11.lib(path)
            logger.info("Library loading success")
            break
        except:
            logger.info("Library loading failed")
    if not LIB:
        raise DevicePKCS11Error(
            "Cannot load library from {path}. ".format(path=pkcs_module_path)
            + "Check path and its format (bitness, endianness, compiler)."
        )

    logger.info(
        "Library information: {} {} {}".format(
            LIB.library_description, LIB.library_version, LIB.manufacturer_id
        )
    )
    if LIB.library_version[1] < 17 and "OpenSC" in LIB.manufacturer_id:
        logger.warning("Issues may occur with this version of library - OpenSC 0.16.")
    return LIB


def get_session(pin=None, token_id=0):
    token_open = None
    token = get_token(token_id)

    error_message = (
        "Cannot open token with ID {token}. Invalid PIN entered or some communication problem "
        "occured with the device. "
        "See if it is inserted or check if the Token ID is correct. "
        "Make sure pcscd (sudo pcscd) is running. Kill and run it again.".format(token=token_id)
    )
    if not token:
        raise DevicePKCS11Error(error_message)

    try:
        if pin:
            token_open = token.open(user_pin=pin, rw=True)
        else:
            token_open = token.open()
    except:
        raise DevicePKCS11Error(error_message)

    return token_open


def import_public_key(key):
    return RSA.importKey(encode_rsa_public_key(key))


def get_public_keys(session):
    try:
        public_keys = list(
            session.get_objects(
                {Attribute.CLASS: ObjectClass.PUBLIC_KEY, Attribute.KEY_TYPE: KeyType.RSA}
            )
        )
    except Exception as e:
        logger.exception("public keys", exc_info=e)
    return public_keys


def get_private_keys(session):
    try:
        priv_keys = list(
            session.get_objects(
                {Attribute.CLASS: ObjectClass.PRIVATE_KEY, Attribute.KEY_TYPE: KeyType.RSA}
            )
        )
    except Exception as e:
        logger.exception("private keys", exc_info=e)
    return priv_keys


def get_token(id=0):
    LIB = get_LIB()

    tokens = list(LIB.get_tokens())
    if len(tokens) > 0:
        return tokens[id]
    else:
        return None


def decrypt_data(pin, token_id, key_id, input_data):
    def _decrypt_data_on_device(encrypted_data, private_key):
        return private_key.decrypt(encrypted_data, mechanism=Mechanism.RSA_PKCS)

    istream = io.BytesIO(input_data)
    ostream = io.BytesIO()
    if not PKCS11_AVAILABLE:
        raise DevicePKCS11Error("PKCS #11 not available !")

    logger.info("Establishing device session")
    with get_session(token_id=token_id, pin=pin) as session:
        logger.info("Getting keys")
        privs = get_private_keys(session)
        if not privs:
            logger.error("No private keys found")
            raise NoKeysFound("No private keys found")

        try:
            key_rsa = privs[key_id]
        except IndexError:
            raise NoKeysFound("Key not found")
        rsa_key_length = key_rsa.key_length
        read_bytes_count = ENCRYPTED_READ_BYTES_COUNT[rsa_key_length]
        logger.info(
            "Got with id {} with {} bits ({})".format(key_rsa.key_type, rsa_key_length, key_rsa)
        )

        key_aes_data = istream.read(read_bytes_count)
        iv_data = istream.read(read_bytes_count)

        logger.info("Reading data")
        data = istream.read(read_bytes_count)
        data_array = []
        while data:
            data_array.append(data)
            data = istream.read(read_bytes_count)

        logger.info("Decrypting AES key")
        # decrypt AES key
        key_aes_data = _decrypt_data_on_device(key_aes_data, key_rsa)
        iv_data = _decrypt_data_on_device(iv_data, key_rsa)
        key_aes = AES.new(key_aes_data, AES.MODE_CBC, iv_data)

        logger.info("Decrypting data using AES key")
        # decrypt data with AES
        data_len = 0
        from tqdm import tqdm

        for data in tqdm(data_array, desc="Decrypting data", maxinterval=0.5):
            decrypted = key_aes.decrypt(data)
            ostream.write(decrypted)
            data_len = +len(decrypted)
        logger.info("Written {} bytes".format(data_len))
        logger.info("Closing files")
        istream.close()
        output_data = ostream.getvalue()
        ostream.close()
        return output_data


def encrypt_data(token_id, key_id, input_data):
    def _encrypt_data(data_to_encrypt_, public_key):
        cipher = PKCS1_v1_5.new(public_key)
        crypttext = cipher.encrypt(data_to_encrypt_)
        return crypttext

    istream = io.BytesIO(input_data)
    ostream = io.BytesIO()
    if not PKCS11_AVAILABLE:
        raise DevicePKCS11Error("PKCS #11 not available !")

    logger.info("Establishing device session")
    with get_session(token_id=token_id) as session:
        logger.info("Getting keys")
        pubs = get_public_keys(session)
        if not pubs:
            logger.error("No public keys found")
            raise NoKeysFound("No public keys found")
        try:
            key_rsa = pubs[key_id]
        except IndexError:
            raise NoKeysFound("Key not found")
        logger.info(
            "Got with id {} with {} bits ({})".format(key_rsa.key_type, key_rsa.key_length, key_rsa)
        )
        rsa_key_length = key_rsa.key_length
        key_rsa = import_public_key(key_rsa)

        logger.info("Creating AES key")
        rndfile = Random.new()
        iv = rndfile.read(16)
        key_aes_data = rndfile.read(32)

        key_aes = AES.new(key_aes_data, AES.MODE_CBC, iv)

        logger.info("Writing encrypted key")
        encrypted = _encrypt_data(key_aes_data, key_rsa)
        ostream.write(encrypted)
        logger.info("Written {} bytes".format(len(encrypted)))
        encrypted = _encrypt_data(iv, key_rsa)
        ostream.write(encrypted)
        logger.info("Written {} bytes".format(len(encrypted)))

        logger.info("Encrypting data")
        data = istream.read(PLAINTEXT_READ_BYTES_COUNT[rsa_key_length])
        data_len = len(data)
        while data:
            divisor = 16
            if len(data) == 0:
                break
            elif len(data) % divisor != 0:
                data += b" " * (divisor - len(data) % divisor)
            encrypted = key_aes.encrypt(data)
            ostream.write(encrypted)
            data = istream.read(PLAINTEXT_READ_BYTES_COUNT[rsa_key_length])
            data_len += len(data)

        logger.info("Written {} bytes".format(data_len))
        logger.info("Closing files")
        istream.close()
        output_data = ostream.getvalue()
        ostream.close()
        return output_data
