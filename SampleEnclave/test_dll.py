# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from ctypes import cdll

LibSgx = cdll.LoadLibrary("./app.dll")
print(LibSgx)
print(LibSgx.initialize_enclave())
print(LibSgx.ecall_my_function())
