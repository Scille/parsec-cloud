import sqlite3

from parsec.core.base import BaseAsyncComponent


class LocalStorage(BaseAsyncComponent):
    def __init__(self, path):
        super().__init__()
        self.path = path
        self.conn = None

    async def _init(self, nursery):
        self.conn = sqlite3.connect(self.path)
        cur = self.conn.cursor()
        cur.execute(
            """
            CREATE TABLE IF NOT EXISTS manifests (
                id TEXT NOT NULL,
                blob BLOB NOT NULL,
                PRIMARY KEY (id)
            )"""
        )
        cur.execute(
            """
            CREATE TABLE IF NOT EXISTS blocks (
                id TEXT NOT NULL,
                blob BLOB NOT NULL,
                PRIMARY KEY (id)
            )"""
        )
        cur.execute(
            """
            CREATE TABLE IF NOT EXISTS users (
                user_id TEXT NOT NULL,
                pubkey BLOB NOT NULL,
                PRIMARY KEY (user_id)
            )"""
        )
        cur.execute(
            """
            CREATE TABLE IF NOT EXISTS devices (
                user_id TEXT NOT NULL,
                device_name TEXT NOT NULL,
                verifykey BLOB NOT NULL,
                PRIMARY KEY (user_id, device_name)
            )"""
        )
        self.conn.commit()

    async def _teardown(self):
        if self.conn:
            self.conn.close()
            self.conn = None

    def fetch_user_manifest(self):
        cur = self.conn.cursor()
        cur.execute('SELECT blob FROM manifests WHERE id="0"')
        try:
            return cur.fetchone()[0]

        except TypeError:
            return None

    def flush_user_manifest(self, blob):
        cur = self.conn.cursor()
        cur.execute('INSERT OR REPLACE INTO manifests (id, blob) VALUES ("0", ?)', (blob,))
        self.conn.commit()

    def fetch_manifest(self, id):
        cur = self.conn.cursor()
        cur.execute("SELECT blob FROM manifests WHERE id=?", (id,))
        try:
            return cur.fetchone()[0]

        except TypeError:
            return None

    def flush_manifest(self, id, blob):
        cur = self.conn.cursor()
        cur.execute("INSERT OR REPLACE INTO manifests (id, blob) VALUES (?, ?)", (id, blob))
        self.conn.commit()

    def move_manifest(self, id, new_id):
        cur = self.conn.cursor()
        cur.execute("UPDATE manifests SET id=? WHERE id=?", (new_id, id))
        self.conn.commit()

    def remove_manifest_local_data(self, id):
        cur = self.conn.cursor()
        cur.execute("DELETE FROM manifests WHERE id=?", (id,))
        self.conn.commit()

    def fetch_block(self, id):
        cur = self.conn.cursor()
        cur.execute("SELECT blob FROM blocks WHERE id=?", (id,))
        res = cur.fetchone()
        if res is not None:
            return res[0]

    def flush_block(self, id, blob):
        cur = self.conn.cursor()
        cur.execute("INSERT OR REPLACE INTO blocks (id, blob) VALUES (?, ?)", (id, blob))
        self.conn.commit()

    def fetch_dirty_block(self, id):
        cur = self.conn.cursor()
        cur.execute("SELECT blob FROM blocks WHERE id=?", (id,))
        res = cur.fetchone()
        if res is not None:
            return res[0]

    def flush_dirty_block(self, id, blob):
        cur = self.conn.cursor()
        cur.execute("INSERT OR REPLACE INTO blocks (id, blob) VALUES (?, ?)", (id, blob))
        self.conn.commit()

    def fetch_user_pubkey(self, user_id):
        cur = self.conn.cursor()
        cur.execute("SELECT pubkey FROM users WHERE user_id=?", (user_id,))
        res = cur.fetchone()
        if res is not None:
            return res[0]

    def flush_user_pubkey(self, user_id, pubkey):
        cur = self.conn.cursor()
        cur.execute(
            "INSERT OR REPLACE INTO users (user_id, pubkey) VALUES (?, ?)", (user_id, pubkey)
        )
        self.conn.commit()

    def fetch_device_verifykey(self, user_id, device_name):
        cur = self.conn.cursor()
        cur.execute(
            "SELECT verifykey FROM devices WHERE user_id=? AND device_name=?",
            (user_id, device_name),
        )
        res = cur.fetchone()
        if res is not None:
            return res[0]

    def flush_device_verifykey(self, user_id, device_name, verifykey):
        cur = self.conn.cursor()
        cur.execute(
            "INSERT OR REPLACE INTO devices (user_id, device_name, verifykey) VALUES (?, ?, ?)",
            (user_id, device_name, verifykey),
        )
        self.conn.commit()
