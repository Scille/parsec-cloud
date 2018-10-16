class BaseMountpointManager:
    def __init__(self, fs, event_bus, mode="thread", debug: bool = True, nothreads: bool = False):
        raise NotImplementedError()

    def get_abs_mountpoint(self):
        raise NotImplementedError()

    async def init(self, nursery):
        raise NotImplementedError()

    def is_started(self):
        raise NotImplementedError()

    async def start(self, mountpoint):
        raise NotImplementedError()

    async def stop(self):
        raise NotImplementedError()

    async def teardown(self):
        raise NotImplementedError()
