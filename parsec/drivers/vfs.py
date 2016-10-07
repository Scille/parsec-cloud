from io import StringIO


class vInode:

    """ Class used to represent the tree view of the VFS. Probably not the best way
    to represent a filesystem but the simplest to implement for the POC
    """

    def __init__(self, name, meta_headers, metadata):
        """ Create a vInode with name and metadata. Initialise children dict if
        current node is supposed to be a directory """
        self._name = name
        self._metadata = {x: v for (x, v) in zip(meta_headers, metadata)}
        if self._metadata.get('type') == 'folder':
            self._children = {}

    def __hash__(self):
        return hash(self._name)

    def __getitem__(self, name):
        """ Returns:
            Child node with the name given in paramter.
            Raise:
                KeyError if child is not in set."""
        return self._children[name]

    def insert(self, path, **kwargs):
        """ Insert a new node in the vInode tree.
        Returns:
            None
        Raises:
            None"""
        head, tail = path.split('/', 1)
        if head in (None, ''):
            self._children[tail] = vInode(tail, kwargs['meta_headers'], kwargs['headers'])
        else:
            child = self._children.get(head)
            if child is None:
                child = vInode(head, meta_headers={}, metadata={})
                self._children[head] = child
            child.insert(tail, **kwargs)

    def find(self, path):
        """ Look for a specific path in the VFS Tree. If path does not exists it raises KeyError.
        Returns:
            metadata of the files
        Raises:
            KeyError if path is not found"""
        head, tail = path.split('/', 1)
        if head in ('', None):
            return self._children[tail]._metadata
        else:
            return self._children[head].find(tail)


class ParsecVFSException(Exception):
    pass


class ParsecVFS:

    """ Virtual FileSystem used by Parsec. In order to be used by other modules
    a glue layer must be implemented in ordre to link the module to the VFS.
    eg : map fuse functions and metadata to their equivalent in the VFS."""

    def __init__(self, driver):
        """ Initialise the filesystem from a driver. The driver must implment a function
        that return the plain manifest files as a StringIO. """
        self._driver = driver
        self._build_root()

    def _build_root(self):
        """ Ask the driver for the manifest file and loads it into memory.
        This method can be called as many time as required. """
        self._root = vInode('/', ('name',), ('/',))
        for line in StringIO(self._driver.get_manifest).readlines():
            path, metadata = line.split(':', 1)
            self._root.insert(path, metadata)

    def _locate_file(self, path):
        try:
            return self._root.find(path)
        except KeyError:
            raise ParsecVFSException('File not found')

    def create_file(self, path, content, metdata):
        pass

    def write_file(self, path, content, metdata):
        pass

    def delete_file(self, path):
        pass

    def stat_file(self, path):
        pass

    def list_dir(self, path):
        pass

    def create_dir(self, path):
        pass
