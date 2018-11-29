# """
# Basically a big hack to share certificates between processes
# """

# import trustme
# import pickle


# def cert_builder(cert_path="localhost.cert", ca_path="localhost.ca"):
#     try:
#         with open(cert_path, "rb") as fd:
#             cert = pickle.load(fd)
#         with open(ca_path, "rb") as fd:

#             class CA:
#                 def __init__(self, cert_pem):
#                     self.cert_pem = cert_pem

#                 def configure_trust(self, ctx):
#                     return trustme.CA.configure_trust(self, ctx)

#             return cert, CA(pickle.load(fd))
#     except:
#         ca = trustme.CA()
#         cert = ca.issue_server_cert("localhost")
#         with open(cert_path, "wb") as fd:
#             pickle.dump(cert, fd)
#         with open(ca_path, "wb") as fd:
#             pickle.dump(ca.cert_pem, fd)
#         return cert, ca
