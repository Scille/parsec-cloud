import trio
from nacl.public import Box, PublicKey

from parsec.utils import from_jsonb64, ejson_loads
from parsec.fs import FSInvalidPath


class Sharing:
    def __init__(self, device, signal_ns, fs, backend_connection):
        self.signal_ns = signal_ns
        self.fs = fs
        self.backend_connection = backend_connection
        self.device = device
        self.msg_arrived = trio.Event()

    async def _message_listener_task(self, *, task_status=trio.TASK_STATUS_IGNORED):
        task_status.started()
        while True:
            await self.msg_arrived.wait()
            # TODO: should keep a message counter in the user manifest
            # too know which message should be processed here
            rep = await self.backend_connection.send({'cmd': 'message_get'})
            assert rep['status'] == 'ok'
            msg = rep['messages'][-1]

            sender_user_id, sender_device_name = msg['sender_id'].split('@')
            rep = await self.backend_connection.send({
                'cmd': 'user_get', 'user_id': sender_user_id})
            assert rep['status'] == 'ok'
            # TODO: handle crash, handle key validity expiration
            sender_verifykey = PublicKey(
                from_jsonb64(rep['devices'][sender_device_name]['verify_key']))
            box = Box(self.device.user_privkey, sender_verifykey)
            cyphered = from_jsonb64(msg['body'])
            clear_msg = ejson_loads(box.decrypt(cyphered).decode('utf8'))
            # TODO: handle other type of message
            assert clear_msg['type'] == 'share'
            sharing_access = self.fs._vlob_access_cls.load(**clear_msg['content'])

            shared_with_folder_name = 'shared-with-%s' % sender_user_id
            if shared_with_folder_name not in self.fs.root:
                shared_with_folder = await self.fs.root.create_folder(shared_with_folder_name)
            else:
                shared_with_folder = await self.fs.root.fetch_child(shared_with_folder_name)
            sharing_name = clear_msg['name']
            while True:
                try:
                    child = await shared_with_folder.insert_child_as_access(
                        sharing_name, sharing_access)
                except FSInvalidPath:
                    sharing_name += '-dup'

            self.signal_ns.signal('new_sharing').send(child.path)

    async def init(self, nursery):
        self._message_listener_task_cancel_scope = nursery.start(self._message_listener_task)
        await self.backend_connection.subscribe_event('message_arrived', self.device.user_id)
        self.signal_ns.signal('message_arrived').connect(self.msg_arrived.set, weak=True)

    async def teardown(self):
        self._message_listener_task_cancel_scope.cancel()
