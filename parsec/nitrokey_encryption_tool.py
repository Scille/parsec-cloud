import sys
import getpass
import logging

try:
    import pkcs11

    from Crypto import Random
    from pkcs11 import KeyType, ObjectClass, Mechanism
    from pkcs11.util.rsa import encode_rsa_public_key

    from Crypto.PublicKey import RSA
    from Crypto.Cipher import PKCS1_v1_5, AES

    NITROKEY_AVAILABLE = True

except ImportError:
    NITROKEY_AVAILABLE = False


VERBOSE = False

try:
    from config_handler import config_instance
    from log_handling import get_logger
    from i18n import tr

    pkcs_module_path = config_instance["PKCS11_MODULE"]
except:

    def tr(x):
        return x

    def get_logger(x):
        return logging.getLogger("main")

    pkcs_module_path = [
        "/usr/lib/opensc-pkcs11.so",
        "/usr/lib/x86_64-linux-gnu/pkcs11/opensc-pkcs11.so",
        "/usr/local/lib/pkcs11/opensc-pkcs11.so",
    ]

# consts for 4096
ERROR_MESSAGE = "Make sure device is inserted and the keys are generated already (RSA2048 or RSA " "4096; best with GPG). " "You may want to change the device or the key-pair id - see --help."
ENCRYPTED_READ_BYTES_COUNT = {2048: 256, 4096: 256 * 2}
# consts for 2048
PLAINTEXT_READ_BYTES_COUNT = {2048: 240, 4096: 240 * 2}

user_pin = None
log = get_logger("encryption")


def test_all_private_keys_to_decrypt(crypttext, priv_keys):
    plaintext = ""
    for priv_key in priv_keys:
        try:
            plaintext = decrypt_on_device(crypttext, priv_key)
            print(repr(plaintext))
            print(priv_key)
            if plaintext:
                break
        except Exception as e:
            print(priv_key)
            log.exception("failed with {}".format(priv_key), exc_info=e)
            pass
    return plaintext


def get_session(pin=None, tokenID=0, skipPin=False):
    LIB = get_LIB()

    token_open = None
    token = get_token(LIB, tokenID)

    error_message = (
        "Cannot open token with ID {token}. Invalid PIN entered or some communication problem "
        "occured with the device. "
        "See if it is inserted or check if the Token ID is correct. "
        "Make sure pcscd (sudo pcscd) is running. Kill and run it again.".format(token=tokenID)
    )
    if not token:
        raise RuntimeError(error_message)

    try:
        if skipPin:
            token_open = token.open()
        else:
            if not pin:
                pin = get_PIN()
            token_open = token.open(user_pin=pin, rw=True)
    except:
        raise RuntimeError(error_message)

    return token_open


LIB = None


def get_LIB():
    global LIB
    if LIB:
        return LIB
    for path in pkcs_module_path:
        try:
            log.info("Loading library from " + path)
            LIB = pkcs11.lib(path)
            log.info("Library loading success")
            break
        except:
            log.info("Library loading failed")
    if not LIB:
        raise RuntimeError(
            "Cannot load library from {path}. ".format(path=pkcs_module_path)
            + "Check path and its format (bitness, endianness, compiler)."
        )

    log.info(
        "Library information: {} {} {}".format(
            LIB.library_description, LIB.library_version, LIB.manufacturer_id
        )
    )
    if LIB.library_version[1] < 17 and "OpenSC" in LIB.manufacturer_id:
        log.warning("Issues may occur with this version of library - OpenSC 0.16.")
    return LIB


def get_PIN():
    global user_pin
    if not user_pin:
        user_pin = getpass.getpass(
            tr("Please provide user PIN for the device (will not be echoed): ")
        )
    return user_pin


def import_public_key(key):
    return RSA.importKey(encode_rsa_public_key(key))


def decrypt_on_device(encrypted_data, private_key) -> bytes:
    def spin_that(q):
        from progress.spinner import Spinner

        b = Spinner("Decrypting data with the device ", file=sys.stderr)
        for i in range(40):
            b.next()
            try:
                if q.get(block=True, timeout=0.5):
                    break
            except:
                pass
        b.finish()
        sys.stderr.write(" done\n")

    # TODO make a decorator from this
    from multiprocessing import Process, Queue

    q = Queue()
    t = Process(target=spin_that, args=(q,))
    t.start()
    try:
        res = _decrypt_on_device(encrypted_data, private_key)
    except:
        q.put("stop")
        t.join()
        raise
    q.put("stop")
    t.join()
    return res


def _decrypt_on_device(encrypted_data, private_key):
    plaintext = private_key.decrypt(encrypted_data, mechanism=Mechanism.RSA_PKCS)
    return plaintext


def encrypt_data_device(data_to_encrypt_, public_key) -> bytes:
    # Encryption on the local machine
    cipher = PKCS1_v1_5.new(public_key)
    crypttext = cipher.encrypt(data_to_encrypt_)
    return crypttext


def create_new_key_pair(session, label="Encryption RSA keypair", rsa_bits=2048):
    # create_new_key_pair(session)
    pub, priv = session.generate_keypair(pkcs11.KeyType.RSA, rsa_bits, store=True, label=label)
    return pub, priv


def get_public_keys(session):
    # Extract public key
    # key = session.get_key(key_type=KeyType.RSA,
    #                       object_class=ObjectClass.PUBLIC_KEY)
    try:
        public_keys = list(
            session.get_objects(
                {
                    pkcs11.Attribute.CLASS: ObjectClass.PUBLIC_KEY,
                    pkcs11.Attribute.KEY_TYPE: KeyType.RSA,
                }
            )
        )
    except Exception as e:
        log.exception("public keys", exc_info=e)
    return public_keys


def get_private_keys(session):
    # Decryption in the HSM
    # priv = session.get_key(key_type=KeyType.RSA,
    #                             object_class=ObjectClass.PRIVATE_KEY)
    try:
        priv_keys = list(
            session.get_objects(
                {
                    pkcs11.Attribute.CLASS: ObjectClass.PRIVATE_KEY,
                    pkcs11.Attribute.KEY_TYPE: KeyType.RSA,
                }
            )
        )
    except Exception as e:
        log.exception("private keys", exc_info=e)
    return priv_keys


def get_token(LIB, id=0):
    tokens = list(LIB.get_tokens())
    if len(tokens) > 0:
        return tokens[id]
    else:
        return None


def get_token_from_names(LIB):
    token = None
    names = ["SmartCard-HSM (UserPIN)", "OpenPGP card (User PIN)", "Nitrokey (UserPIN)", ""]
    for name in names:
        try:
            log.info("Trying to use token with label: {}".format(name))
            token = LIB.get_token(token_label=name)
            break
        except:
            pass
    if not token:
        log.error("No token detected")
        sys.exit(1)
    return token


def decrypt_data(tokenID, pin, istream, ostream, keyid):
    if not NITROKEY_AVAILABLE:
        raise RuntimeError("Nitrokey not available !")

    log.info("Establishing device session")
    with get_session(tokenID=tokenID, pin=pin) as session:
        log.info("Getting keys")
        privs = get_private_keys(session)
        if not privs:
            log.error("No private keys found")
            raise NoKeysFound

        key_rsa = privs[keyid]
        rsa_key_length = key_rsa.key_length
        read_bytes_count = ENCRYPTED_READ_BYTES_COUNT[rsa_key_length]
        log.info(
            "Got with id {} with {} bits ({})".format(key_rsa.key_type, rsa_key_length, key_rsa)
        )

        key_aes_data = istream.read(read_bytes_count)
        iv_data = istream.read(read_bytes_count)

        log.info("Reading data")
        data = istream.read(read_bytes_count)
        data_array = []
        while data:
            data_array.append(data)
            data = istream.read(read_bytes_count)

        log.info("Decrypting AES key")
        # decrypt AES key
        key_aes_data = decrypt_on_device(key_aes_data, key_rsa)
        iv_data = decrypt_on_device(iv_data, key_rsa)
        key_aes = AES.new(key_aes_data, AES.MODE_CBC, iv_data)

        log.info("Decrypting data using AES key")
        # decrypt data with AES
        data_len = 0
        from tqdm import tqdm

        for data in tqdm(data_array, desc="Decrypting data", maxinterval=0.5):
            decrypted = key_aes.decrypt(data)
            ostream.write(decrypted)
            data_len = +len(decrypted)
        log.info("Written {} bytes".format(data_len))
        log.info("Closing files")
        # istream.close()
        # ostream.close()
        print(" ", file=sys.stderr)


class NoKeysFound(Exception):
    pass


def encrypt_data(istream, ostream, keyid: int, token: int):
    if not NITROKEY_AVAILABLE:
        raise RuntimeError("Nitrokey not available !")

    log.info("Establishing device session")
    with get_session(tokenID=token, skipPin=True) as session:
        log.info("Getting keys")
        pubs = get_public_keys(session)
        if not pubs:
            log.error("No public keys found")
            raise NoKeysFound
        key_rsa = pubs[keyid]
        log.info(
            "Got with id {} with {} bits ({})".format(key_rsa.key_type, key_rsa.key_length, key_rsa)
        )
        rsa_key_length = key_rsa.key_length
        key_rsa = import_public_key(key_rsa)

        log.info("Creating AES key")
        rndfile = Random.new()
        iv = rndfile.read(16)
        key_aes_data = rndfile.read(32)

        key_aes = AES.new(key_aes_data, AES.MODE_CBC, iv)

        log.info("Writing encrypted key")
        encrypted = encrypt_data_device(key_aes_data, key_rsa)
        ostream.write(encrypted)
        log.info("Written {} bytes".format(len(encrypted)))
        encrypted = encrypt_data_device(iv, key_rsa)
        ostream.write(encrypted)
        log.info("Written {} bytes".format(len(encrypted)))

        log.info("Encrypting data")
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

        log.info("Written {} bytes".format(data_len))
        log.info("Closing files")
        # istream.close()
        # ostream.close()
        print(" ", file=sys.stderr)


def print_status(token: int, pin: str):
    log.info("Printing status data")
    working = []
    with get_session(tokenID=token, pin=pin) as session:
        pubs = get_public_keys(session)
        privs = get_private_keys(session)
        print("Investigating device {}".format(token))
        print("Public/private RSA keys found: {}/{}".format(len(pubs), len(privs)))
        data = os.urandom(16)
        for i in range(len(pubs)):
            dec = bytes()
            enc = bytes()
            print("\nTest key {}: {}".format(i, pubs[i]))
            key_rsa_pub = import_public_key(pubs[i])

            try:
                enc = encrypt_data_device(data, key_rsa_pub)
                dec = decrypt_on_device(enc, privs[i])
            except:
                pass

            print("Encryption/decryption test result with key {}: ".format(i), end="")
            if data.hex() == dec.hex():
                print("success")
                working.append((token, i))
            else:
                print("failure")
                log.info("Data: {}".format(data.hex()))
                log.info("Decrypted: {}".format(dec.hex()))
    return working


if __name__ == "__main__":
    # for local tests

    from argparse import ArgumentParser
    import os.path

    def is_valid_file(parser, arg):
        if not os.path.exists(arg):
            parser.error("The file %s does not exist!" % arg)
        else:
            return open(arg, "rb")  # return an open file handle

    def is_valid_write_file(parser, arg):
        if os.path.exists(arg):
            print("The file %s exists!" % arg, file=sys.stderr)
        return open(arg, "wb")  # return an open file handle

    import argparse

    parser = argparse.ArgumentParser(description="Encrypt/decrypt or generate key through PKCS#11")
    parser.set_defaults(cmd="status")

    subparsers = parser.add_subparsers(
        help="Action to be run by the tool. Each action has its own help (use <action> --help). "
        "Description:"
    )

    sp = subparsers.add_parser("decrypt", help="Decrypt file with private key from the device")
    sp.set_defaults(cmd="decrypt")
    sp.add_argument("input", help="Input file", type=lambda x: is_valid_file(parser, x))
    sp.add_argument("output", help="Output file", type=lambda x: is_valid_write_file(parser, x))
    sp.add_argument(
        "keyid",
        type=int,
        help="Key pair number to work on. Use list_keys to see what key pairs are available",
    )

    sp = subparsers.add_parser("encrypt", help="Encrypt file with public key from the device")
    sp.set_defaults(cmd="encrypt")
    sp.add_argument("input", help="Input file", type=lambda x: is_valid_file(parser, x))
    sp.add_argument("output", help="Output file", type=lambda x: is_valid_write_file(parser, x))

    sp.add_argument(
        "keyid",
        type=int,
        help="Key pair number to work on. Use list_keys to see what key pairs are available",
    )
    sp = subparsers.add_parser("list_keys", help="List public keys saved on token (no PIN needed)")
    sp.set_defaults(cmd="list_public_keys")
    sp = subparsers.add_parser("list_tokens", help="List available tokens")
    sp.set_defaults(cmd="list_tokens")

    sp = subparsers.add_parser(
        "status",
        help="Show what RSA keys are available and perform simple "
        "encryption/decryption test. Default action.",
    )
    sp.set_defaults(cmd="status")

    sp = subparsers.add_parser(
        "create_keys",
        help="Create RSA keys for use in tool. Warning: it will not work with OpenSC in "
        "version 0.16. Only Smartcard-HSM is supported.",
    )
    sp.set_defaults(cmd="create_keys")
    sp.add_argument("label", help="Key label", type=str)
    sp.add_argument("--bits", help="Key bits", type=int, default=2048, choices=[2048, 4096])

    parser.add_argument(
        "--token",
        type=int,
        default=0,
        help="Token number to work on. Use list_tokens to see what tokens are available. (default: 0)",
    )
    parser.add_argument(
        "--pin",
        type=str,
        default=None,
        help="User PIN. Will be asked when needed if not provided through switch.",
    )
    parser.add_argument(
        "--verbose",
        default=False,
        action="store_true",
        help="Show detailed information about current operation.",
    )
    args = parser.parse_args()
    print(args, file=sys.stderr)
    import sys

    def print_public_keys(session):
        pub = get_public_keys(session)
        for i, p in enumerate(pub):
            print("Key pair number: {}. {}".format(i, p))

    if args.verbose:
        VERBOSE = True
        logger = get_logger("main")
        logger.setLevel(logging.DEBUG)
        ch = logging.StreamHandler()
        ch.setLevel(logging.DEBUG)
        ch.setFormatter(logging.Formatter("%(asctime)s - %(levelname)s - %(message)s"))
        logger.addHandler(ch)

    if args.cmd == "list_tokens":
        LIB = get_LIB()
        tokens = list(LIB.get_tokens())
        print("Listing tokens ({}):".format(len(tokens)))
        for i, t in enumerate(tokens):
            print("{}. {}".format(i, t))
        exit(0)

    if args.cmd == "encrypt":
        try:
            encrypt_data(args.input, args.output, args.keyid, args.token)
        except:
            print("Encryption failed. " + ERROR_MESSAGE)

    with get_session(tokenID=args.token, skipPin=True) as session:
        if args.cmd == "list_public_keys":
            print_public_keys(session)

    if args.cmd == "decrypt":
        try:
            decrypt_data(args.token, args.pin, args.input, args.output, args.keyid)
        except:
            print("Decryption failed. " + ERROR_MESSAGE)

    elif args.cmd == "create_keys":
        try:
            with get_session(tokenID=args.token, pin=args.pin) as session:
                create_new_key_pair(session, args.label, args.bits)
                print_public_keys(session)

        except:
            print(
                "Key generation failed. It will not work with OpenSC 0.16 - make sure you are not using it.",
                file=sys.stderr,
                flush=True,
            )
            exit(1)

    elif args.cmd == "status":
        LIB = get_LIB()
        tokens = list(LIB.get_tokens())
        print("Listing tokens ({}):".format(len(tokens)))
        res = []
        for i, t in enumerate(tokens):
            print('\nToken {}. "{}"'.format(i, t))
            res += print_status(i, args.pin)

        print("\nWorking configurations:")
        for tokenid, key in res:
            print('Token {} ("{}"): key {}'.format(tokenid, tokens[tokenid], key))
