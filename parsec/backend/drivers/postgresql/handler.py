import trio
from threading import Thread
from queue import Queue, Empty
from urllib.parse import urlparse
from psycopg2.extensions import parse_dsn
from psycopg2 import connect as pgconnect, Error as PGError


def init_db(url, force=False):
    conn = pgconnect(**parse_dsn(url))
    cursor = conn.cursor()

    if force:
        for table in ('blockstore', 'groups', 'group_identities', 'messages',
                      'pubkeys', 'users', 'user_devices', 'invitations',
                      'vlobs', 'user_vlobs'):
            cursor.execute('DROP TABLE IF EXISTS %s' % table)

    cursor.execute(
        'CREATE TABLE IF NOT EXISTS blockstore ('
            '_id SERIAL PRIMARY KEY, '
            'id VARCHAR(32), '
            'block BYTEA'
        ')'
    )
    cursor.execute(
        'CREATE TABLE IF NOT EXISTS groups ('
            'id SERIAL PRIMARY KEY, '
            'name TEXT UNIQUE'
        ')'
    )
    cursor.execute(
        'CREATE TABLE IF NOT EXISTS group_identities ('
            'id SERIAL PRIMARY KEY, '
            'group_id INTEGER, '
            'name TEXT, '
            'admin BOOLEAN, '
            'UNIQUE (group_id, name)'
        ')'
    )
    cursor.execute(
        'CREATE TABLE IF NOT EXISTS messages ('
            'id SERIAL PRIMARY KEY, '
            'recipient TEXT, '
            'body BYTEA'
        ')'
    )
    cursor.execute(
        'CREATE TABLE IF NOT EXISTS pubkeys ('
            '_id SERIAL PRIMARY KEY, '
            'id VARCHAR(32) UNIQUE, '
            'pubkey BYTEA, '
            'verifykey BYTEA'
        ')'
    )
    cursor.execute(
        'CREATE TABLE IF NOT EXISTS users ('
            '_id SERIAL PRIMARY KEY, '
            'user_id VARCHAR(32) UNIQUE, '
            'created_on INTEGER, '
            'created_by VARCHAR(32), '
            'broadcast_key BYTEA'
        ')'
    )
    cursor.execute(
        'CREATE TABLE IF NOT EXISTS user_devices ('
            '_id SERIAL PRIMARY KEY, '
            'user_id VARCHAR(32), '
            'device_name TEXT, '
            'created_on INTEGER, '
            'verify_key BYTEA, '
            'revocated_on INTEGER'
        ')'
    )
    cursor.execute(
        'CREATE TABLE IF NOT EXISTS invitations ('
            '_id SERIAL PRIMARY KEY, '
            'user_id VARCHAR(32) UNIQUE, '
            'ts INTEGER, '
            'author VARCHAR(32), '
            'invitation_token TEXT, '
            'claim_tries INTEGER'
        ')'
    )
    cursor.execute(
        'CREATE TABLE IF NOT EXISTS vlobs ('
            '_id SERIAL PRIMARY KEY, '
            'id VARCHAR(32), '
            'version INTEGER, '
            'rts TEXT, '
            'wts TEXT, '
            'blob BYTEA '
        ')'
    )
    cursor.execute(
        'CREATE TABLE IF NOT EXISTS user_vlobs ('
            '_id SERIAL PRIMARY KEY, '
            'id VARCHAR(32), '
            'version INTEGER, '
            'blob BYTEA '
        ')'
    )
    conn.commit()
    cursor.close()
    return conn



class PGHandler(Thread):
    def __init__(self, url, signal_ns, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.url = url

        self.signal_ns = signal_ns
        self.signals = [
            'message_arrived',
            'user_claimed',
            'user_vlob_updated',
            'vlob_updated'
        ]

        for signal in self.signals:
            sighandler = self.signal_ns.signal('message_arrived')
            sighandler.connect(self.get_signal_handler(signal))

        self.reqqueue = Queue()
        self.respqueue = Queue()

    def run(self):
        conn = init_db(self.url)
        cursor = conn.cursor()

        for signal in self.signals:
            cursor.execute('LISTEN {0}'.format(signal))

        self.respqueue.put({'status': 'ok'})

        running = True

        while running:
            while True:
                try:
                    # TODO: timeout avoid this threads to eat all the CPU,
                    # but it would be better to have a dedicated thread
                    # for events listening
                    req = self.reqqueue.get(timeout=0.1)

                except Empty:
                    break

                if req['type'] == 'stop':
                    running = False
                    break

                else:
                    try:
                        reqtype = req.pop('type')
                        reqmethod = getattr(self, 'cmd_{0}'.format(reqtype))
                        resp = reqmethod(cursor, **req)

                    except Exception as err:
                        resp = {'status': 'ko', 'error': err}

                    self.respqueue.put(resp)

            conn.poll()
            while conn.notifies:
                notify = conn.notifies.pop(0)
                signal = self.signal_ns.signal(notify.channel)
                signal.send(notify.payload, propagate=False)

        cursor.close()
        conn.close()

        self.respqueue.put({'status': 'ok'})

    def cmd_read_one(self, cursor, sql=None, params=None):
        assert sql is not None
        assert params is not None

        cursor.execute(sql, params)

        return {'status': 'ok', 'data': cursor.fetchone()}

    def cmd_read_many(self, cursor, sql=None, params=None):
        assert sql is not None
        assert params is not None

        cursor.execute(sql, params)

        return {'status': 'ok', 'data': cursor.fetchall()}

    def cmd_write_one(self, cursor, sql=None, params=None):
        assert sql is not None
        assert params is not None

        cursor.execute(sql, params)
        cursor.connection.commit()

        return {'status': 'ok'}

    def cmd_write_many(self, cursor, sql=None, paramslist=None):
        assert sql is not None
        assert paramslist is not None

        for params in paramslist:
            cursor.execute(sql, params)

        cursor.connection.commit()

        return {'status': 'ok'}

    def cmd_notify(self, cursor, signal=None, sender=None):
        cursor.execute('NOTIFY {0}, %s'.format(signal), (sender,))

    async def get_response(self):
        while True:
            try:
                return self.respqueue.get(block=False)

            except Empty:
                await trio.sleep(0.1)

    async def start(self):
        super().start()
        await self.get_response()

    async def stop(self):
        self.reqqueue.put({'type': 'stop'})
        await self.get_response()
        self.join()

    async def fetch_one(self, sql, params):
        self.reqqueue.put({'type': 'read_one', 'sql': sql, 'params': params})
        resp = await self.get_response()

        if resp['status'] != 'ok':
            raise resp.get('error', RuntimeError('Unknown error'))

        return resp['data']

    async def fetch_many(self, sql, params):
        self.reqqueue.put({'type': 'read_many', 'sql': sql, 'params': params})
        resp = await self.get_response()

        if resp['status'] != 'ok':
            raise resp.get('error', RuntimeError('Unknown error'))

        return resp['data']

    async def insert_one(self, sql, params):
        self.reqqueue.put({'type': 'write_one', 'sql': sql, 'params': params})
        resp = await self.get_response()

        if resp['status'] != 'ok':
            raise resp.get('error', RuntimeError('Unknown error'))

    async def insert_many(self, sql, paramslist):
        self.reqqueue.put({'type': 'write_many', 'sql': sql, 'paramslist': paramslist})
        resp = await self.get_response()

        if resp['status'] != 'ok':
            raise resp.get('error', RuntimeError('Unknown error'))

    async def update_one(self, sql, params):
        self.reqqueue.put({'type': 'write_one', 'sql': sql, 'params': params})
        resp = await self.get_response()

        if resp['status'] != 'ok':
            raise resp.get('error', RuntimeError('Unknown error'))

    async def update_many(self, sql, paramslist):
        self.update_one(sql, paramslist)

    async def delete_one(self, sql, params):
        self.reqqueue.put({'type': 'write_one', 'sql': sql, 'params': params})
        resp = await self.get_response()

        if resp['status'] != 'ok':
            raise resp.get('error', RuntimeError('Unknown error'))

    async def delete_many(self, sql, paramslist):
        self.reqqueue.put({'type': 'write_many', 'sql': sql, 'paramslist': paramslist})
        resp = await self.get_response()

        if resp['status'] != 'ok':
            raise resp.get('error', RuntimeError('Unknown error'))

    def get_signal_handler(self, signal):
        def signal_handler(sender, propagate=True):
            if propagate:
                self.reqqueue.put({
                    'type': 'notify',
                    'signal': signal,
                    'sender': sender
                })

        return signal_handler
