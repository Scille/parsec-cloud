"""
Basically a big hack to share certificates between processes
"""

import trustme
import pickle


CERT_PATH = "localhost.cert"
CA_PATH = "localhost.ca"


try:
    with open(CERT_PATH, "rb") as fd:
        CERT = pickle.load(fd)
    with open(CA_PATH, "rb") as fd:

        class CA:
            def __init__(self, cert_pem):
                self.cert_pem = cert_pem

            def configure_trust(self, ctx):
                return trustme.CA.configure_trust(self, ctx)

        CA = CA(pickle.load(fd))
except:
    CA = trustme.CA()
    CERT = CA.issue_server_cert("localhost")
    with open(CERT_PATH, "wb") as fd:
        pickle.dump(CERT, fd)
    with open(CA_PATH, "wb") as fd:
        pickle.dump(CA.cert_pem, fd)
