import pytest
from functools import namedtuple

from parsec.config import (
    LazyObj, LazyRef, _build_obj_from_config, ConfigError, Topology, loads_config)
from parsec.abstract import BaseService, BaseServer, BaseClient


@pytest.fixture
def topology():
    return Topology()


@pytest.fixture
def mock_service(mocker):

    class MockService(BaseService):

        def dispatch_msg(self, msg):
            return None

        def dispatch_raw_msg(self, msg):
            return None

    mocker.patch('parsec.volume.MockService', create=True, new=MockService)
    return namedtuple('MockResume', 'cls path')(MockService, 'volume.MockService')


def test_build_obj(mock_service, topology):
    conf = {
        "id": "services:mock-01",
        "class": mock_service.path
    }
    obj = _build_obj_from_config(conf)
    assert isinstance(obj, LazyObj)
    assert obj.id == "services:mock-01"
    resolved_obj = obj.resolve(topology)
    assert isinstance(resolved_obj, mock_service.cls)


def test_envvar(mocker, mock_service, topology):

    def __init__(self, var: str, no_default: str, casted_var: int, with_default: str='CCC'):
        self.var = var
        self.no_default = no_default
        self.casted_var = casted_var
        self.with_default = with_default

    mock_service.cls.__init__ = __init__
    mocker.patch('os.environ', {'ENV_VAR': 'AAA', 'INT_VAR': '42'})
    conf = {
        "class": mock_service.path,
        "params": {
            'var': '$ENV_VAR',
            'no_default': '$MISSING_VAR',
            'casted_var': '$INT_VAR',
            'with_default': '$MISSING_VAR'
        }
    }
    obj = _build_obj_from_config(conf)
    resolved_obj = obj.resolve(topology)
    assert resolved_obj.var == 'AAA'
    assert resolved_obj.no_default == ''
    assert resolved_obj.casted_var == 42
    assert resolved_obj.with_default == 'CCC'


def test_build_obj_params(mock_service, topology):

    def __init__(self, path: str, count: int, with_default: int=1):
        self.path = path
        self.count = count
        self.with_default = with_default

    mock_service.cls.__init__ = __init__
    conf = {
        "id": "services:mock-01",
        "class": mock_service.path,
        "params": {"path": "/foo", "count": 42}
    }
    obj = _build_obj_from_config(conf)
    assert isinstance(obj, LazyObj)
    assert obj.id == "services:mock-01"
    resolved_obj = obj.resolve(topology)
    assert isinstance(resolved_obj, mock_service.cls)
    assert resolved_obj.path == '/foo'
    assert resolved_obj.count == 42
    assert resolved_obj.with_default == 1


def test_nested_build_obj(mocker, mock_service, topology):

    class MockNested:

        def __init__(self, count: int):
            self.count = count

    def __init__(self, nested: MockNested):
        self.nested = nested

    mock_service.cls.__init__ = __init__

    mocker.patch('parsec.volume.MockNested', create=True, new=MockNested)
    conf = {
        "id": "services:mock-01",
        "class": mock_service.path,
        "params": {"nested": {"class": "volume.MockNested", "params": {"count": 42}}}
    }
    obj = _build_obj_from_config(conf)
    assert isinstance(obj, LazyObj)
    assert obj.id == "services:mock-01"
    resolved_obj = obj.resolve(topology)
    assert isinstance(resolved_obj, mock_service.cls)
    assert isinstance(resolved_obj.nested, MockNested)
    assert resolved_obj.nested.count == 42


# useful helper
class missing:
    def __repr__(self):
        return "<missing>"


@pytest.mark.parametrize("bad_conf", (
    {"class": missing},
    {"id": 42}, {"id": {}}, {"dummy": "field"},
    {"params": {"path": "/foo", "dummy": 42}},
    {"params": 42}, {"params": [{"path": "/foo"}]},
    {"class": 42}, {"class": "unknown.MissingMe"}
))
def test_bad_build_obj_params(mock_service, bad_conf):

    def __init__(self, path: str):
        self.path = path

    mock_service.cls.__init__ = __init__
    conf_template = {
        "id": "services:mock-01",
        "class": mock_service.path,
        "params": {"path": "/foo"}
    }
    conf = conf_template.copy()
    conf.update(bad_conf)
    conf = {k: v for k, v in conf.items() if v is not missing}
    with pytest.raises(ConfigError) as exc:
        _build_obj_from_config(conf)
    print(exc)


@pytest.mark.parametrize("bad_nested_params", (
    {"class": "unknown.MissingMe", "params": {"count": 1}},
    {"class": "volume.MockNested", "params": {"count": "dummy"}},
    {"class": "volume.MockNested", "params": missing},
    {"class": "volume.MockNested", "params": {"count": 1, "dummy": "field"}},
    {"class": "volume.MockNested", "params": {"count": 1, "with_default": "dummy"}}
))
def test_bad_build_obj_params_nested(mocker, bad_nested_params):

    class MockNested:

        def __init__(self, count: int, with_default: int=42):
            self.count = count
            self.with_default = with_default

    class MockService(BaseService):

        def __init__(self, nested: MockNested):
            self.nested = nested

        def dispatch_msg(self, msg):
            return None

        def dispatch_raw_msg(self, msg):
            return None

    mocker.patch('parsec.volume.MockNested', create=True, new=MockNested)
    mocker.patch('parsec.volume.MockService', create=True, new=MockService)
    conf_template = {
        "id": "services:mock-01",
        "class": "volume.MockService",
        "params": {"nested": {"class": "volume.MockNested", "params": {"count": 1}}}
    }
    conf = conf_template.copy()
    nested_conf = conf['params']['nested']
    nested_conf.update(bad_nested_params)
    nested_conf = {k: v for k, v in nested_conf.items() if v is not missing}
    conf['params']['nested'] = nested_conf
    with pytest.raises(ConfigError) as exc:
        _build_obj_from_config(conf)
    print(exc)


def test_lazy_ref(topology, mock_service):
    service = mock_service.cls()
    topology.add_service('services:mock-01', service)

    ref = LazyRef('services:mock-01')
    obj = ref.resolve(topology)
    assert obj is service


def test_bad_lazy_ref(topology, mock_service):
    service = mock_service.cls()
    topology.add_service('services:mock-01', service)

    ref = LazyRef('bad-id')
    with pytest.raises(ConfigError) as exc:
        ref.resolve(topology)
    print(exc)


def test_retrieve_obj(mocker, topology):

    class MockNested:
        pass

    class MockService(BaseService):

        def __init__(self, nested: MockNested):
            self.nested = nested

        def dispatch_msg(self, msg):
            return None

        def dispatch_raw_msg(self, msg):
            return None

    mocker.patch('parsec.volume.MockService', create=True, new=MockService)

    nested = MockNested()
    topology.add_service('nested-01', nested)
    conf = {
        "id": "services:mock-01",
        "class": "volume.MockService",
        "params": {'nested': 'nested-01'}
    }
    obj = _build_obj_from_config(conf)
    assert isinstance(obj, LazyObj)
    assert obj.id == "services:mock-01"
    resolved_obj = obj.resolve(topology)
    assert isinstance(resolved_obj, MockService)
    assert resolved_obj.nested is nested


@pytest.mark.parametrize("bad_id", ('bad-id', None, 42))
def test_bad_retrieve_obj(mocker, topology, bad_id):

    class MockNested:
        pass

    class MockService(BaseService):

        def __init__(self, nested: MockNested):
            self.nested = nested

        def dispatch_msg(self, msg):
            return None

        def dispatch_raw_msg(self, msg):
            return None

    mocker.patch('parsec.volume.MockService', create=True, new=MockService)

    nested = MockNested()
    topology.add_service('nested-01', nested)
    conf = {
        "id": "services:mock-01",
        "class": "volume.MockService",
        "params": {'nested': bad_id}
    }
    with pytest.raises(ConfigError) as exc:
        obj = _build_obj_from_config(conf)
        obj.resolve(topology)
    print(exc)


def test_loads_config(mocker):

    class MockService(BaseService):

        def __init__(self, a: int=42):
            self.a = a

        def dispatch_msg(self, msg):
            return None

        def dispatch_raw_msg(self, msg):
            return None

    class MockClient(BaseClient):

        def __init__(self, service: MockService):
            self.service = service

        def _ll_communicate(self, **kwargs):
            pass

        @property
        def request_cls(self):
            return None

        @property
        def response_cls(self):
            return None

    class MockServer(BaseServer):

        def start(self):
            pass

        def stop(self):
            pass

    mocker.patch('parsec.volume.MockService', create=True, new=MockService)
    mocker.patch('parsec.volume.MockClient', create=True, new=MockClient)
    mocker.patch('parsec.broker.MockServer', create=True, new=MockServer)

    config = """
services:
- id: "services:1"
  class: volume.MockService
- id: "services:2"
  class: volume.MockService
  params:
    a: 1
clients:
- id: "clients:1"
  class: volume.MockClient
  params:
    service: "services:1"
servers:
- id: "servers:1"
  class: broker.MockServer
"""
    topology = loads_config(config)

    assert len(topology.services_factory) == 2
    service2 = topology.services_factory['services:2']()
    service1 = topology.services_factory['services:1']()
    assert isinstance(service1, MockService)
    assert isinstance(service2, MockService)
    assert service1.a == 42
    assert service2.a == 1

    assert len(topology.clients_factory) == 1
    client = topology.clients_factory['clients:1']()
    assert isinstance(client, MockClient)
    assert client.service is service1

    assert len(topology.servers_factory) == 1
    server = topology.servers_factory['servers:1']()
    assert isinstance(server, MockServer)
