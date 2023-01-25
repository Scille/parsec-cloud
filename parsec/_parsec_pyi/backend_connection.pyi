from oxidation.bindings.generator.api import DeviceID
from parsec._parsec_pyi.addrs import BackendOrganizationAddr
from parsec._parsec_pyi.crypto import SigningKey
from parsec._parsec_pyi.protocol.ping import AuthenticatedPingRep

class AuthenticatedCmds:
    def __init__(
        self, server_url: BackendOrganizationAddr, device_id: DeviceID, signing_key: SigningKey
    ) -> None: ...
    async def ping(self, ping: str) -> AuthenticatedPingRep: ...
