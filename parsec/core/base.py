from interface import Interface, implements


class IAsyncComponent(Interface):

    async def init(self, nursery):
        pass

    async def teardown(self):
        pass
