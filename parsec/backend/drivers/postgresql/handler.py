from psycopg2.extensions import parse_dsn
from psycopg2 import connect as pgconnect, Error as PGError

from threading import Thread
from queue import Queue, Empty
import trio


class PGHandler(Thread):
    def __init__(self, url, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.url = url
        self.reqqueue = Queue()
        self.respqueue = Queue()

    def run(self):
        conn = pgconnect(**parse_dsn(self.url))
        cursor = conn.cursor()

        cursor.execute(
            'CREATE TABLE IF NOT EXIST blockstore ('
                '_id INTEGER PRIMARY KEY, '
                'id VARCHAR(32), block TEXT UNIQUE'
            ')'
        )
        cursor.execute(
            'CREATE TABLE IF NOT EXIST groups ('
                'id INTEGER PRIMARY KEY, '
                'name TEXT UNIQUE'
            ')'
        )
        cursor.execute(
            'CREATE TABLE IF NOT EXIST group_identities ('
                'id INTEGER PRIMARY KEY, '
                'group_id INTEGER, '
                'name TEXT, '
                'admin BOOLEAN, '
                'UNIQUE (group_id, name)'
            ')'
        )
        cursor.execute(
            'CREATE TABLE IF NOT EXIST messages ('
                'id INTEGER PRIMARY KEY, '
                'recipient TEXT, '
                'body TEXT'
            ')'
        )
        cursor.execute(
            'CREATE TABLE IF NOT EXIST pubkeys ('
                '_id INTEGER PRIMARY KEY, '
                'id VARCHAR(32) UNIQUE, '
                'pubkey BYTEA, '
                'verifykey BYTEA'
            ')'
        )
        cursor.execute(
            'CREATE TABLE IF NOT EXIST users ('
                '_id INTEGER PRIMARY KEY, '
                'id VARCHAR(32) UNIQUE, '
                'created_on INTEGER, '
                'created_by VARCHAR(32), '
                'broadcast_key BYTEA'
            ')'
        )
        cursor.execute(
            'CREATE TABLE IF NOT EXIST user_devices ('
                'id INTEGER PRIMARY KEY, '
                'name TEXT, '
                'created_on INTEGER, '
                'verify_key BYTEA, '
                'revocated_on INTEGER'
            ')'
        )
        cursor.execute(
            'CREATE TABLE IF NOT EXIST invitations ('
                '_id INTEGER PRIMARY KEY, '
                'id VARCHAR(32) UNIQUE, '
                'ts INTEGER, '
                'author VARCHAR(32), '
                'token TEXT, '
                'claim_tries INTEGER'
            ')'
        )
        cursor.execute(
            'CREATE TABLE IF NOT EXIST vlobs ('
                '_id INTEGER PRIMARY KEY, '
                'id VARCHAR(32), '
                'rts TEXT, '
                'wts TEXT, '
                'blob BYTEA '
            ')'
        )
        conn.commit()

        self.respqueue.put({'status': 'ok'})

        while True:
            req = self.reqqueue.get()

            if req['type'] == 'stop':
                break

            else:
                try:
                    reqmethod = getattr(self, 'cmd_{0}'.format(req['type']))
                    resp = reqmethod(cursor, **req)

                except Exception as err:
                    resp = {'status': 'ko', 'error': err}

                self.respqueue.put(resp)

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

    async def insert_many(self, sql, params):
        self.reqqueue.put({'type': 'write_many', 'sql': sql, 'params': params})
        resp = await self.get_response()

        if resp['status'] != 'ok':
            raise resp.get('error', RuntimeError('Unknown error'))

    async def update_one(self, sql, params):
        self.reqqueue.put({'type': 'write_one', 'sql': sql, 'params': params})
        resp = await self.get_response()

        if resp['status'] != 'ok':
            raise resp.get('error', RuntimeError('Unknown error'))

    async def update_many(self, sql, params):
        self.update_one(sql, params)

    async def delete_one(self, sql, params):
        self.reqqueue.put({'type': 'write_one', 'sql': sql, 'params': params})
        resp = await self.get_response()

        if resp['status'] != 'ok':
            raise resp.get('error', RuntimeError('Unknown error'))

    async def delete_many(self, sql, params):
        self.reqqueue.put({'type': 'write_many', 'sql': sql, 'params': params})
        resp = await self.get_response()

        if resp['status'] != 'ok':
            raise resp.get('error', RuntimeError('Unknown error'))
