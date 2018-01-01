import sqlite3


class BaseLocalStorage:
    def __init__(self, user):
        self.user = user

    def fetch_user_manifest(self):
        raise NotImplementedError()

    def flush_user_manifest(self, blob):
        raise NotImplementedError()

    def fetch_manifest(self, id):
        raise NotImplementedError()

    def flush_manifest(self, id, blob):
        raise NotImplementedError()

    def move_manifest(self, id, new_id):
        raise NotImplementedError()


class LocalStorage(BaseLocalStorage):
    def __init__(self, path):
        self.path = path
        self.conn = None

    def init(self):
        self.conn = sqlite3.connect(self.path)
        cur = self.conn.cursor()
        cur.execute("CREATE TABLE IF NOT EXISTS manifests (id TEXT, blob BLOB)")
        self.conn.commit()

    def teardown(self):
        if self.conn:
            self.conn.close()
            self.conn = None

    def fetch_user_manifest(self):
        cur = self.conn.cursor()
        cur.execute('SELECT blob FROM manifests WHERE id="0"')
        try:
            return cur.fetchone()[0]
        except TypeError:
            return b''

    def flush_user_manifest(self, blob):
        cur = self.conn.cursor()
        cur.execute('INSERT OR REPLACE INTO manifests (id, blob) VALUES ("0", ?)', (blob,))
        self.conn.commit()

    def fetch_manifest(self, id):
        cur = self.conn.cursor()
        cur.execute('SELECT blob FROM manifests WHERE id=?', (id,))
        return cur.fetchone()[0]

    def flush_manifest(self, id, blob):
        cur = self.conn.cursor()
        cur.execute('INSERT OR REPLACE INTO manifests (id, blob) VALUES (?, ?)', (id, blob))
        self.conn.commit()

    def move_manifest(self, id, new_id):
        cur = self.conn.cursor()
        cur.execute('UPDATE blob SET (id=?) WHERE (id=?)', (new_id, id))
        self.conn.commit()
