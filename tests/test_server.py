import pytest

from parsec.service import BaseService, service
from parsec.server import BaseServer


@pytest.fixture
def server():
    return BaseServer()


@pytest.fixture
def services():
    class RootService(BaseService):
        pass

    class NodeAService(BaseService):
        _root = service('RootService')

    class NodeBService(BaseService):
        _root = service('RootService')

    class LeafService(BaseService):
        _nodea = service('NodeAService')
        _nodeb = service('NodeBService')

    return RootService, NodeAService, NodeBService, LeafService


def test_services_dependencies_good(server, services):
    RootService, NodeAService, NodeBService, LeafService = services

    sroot = RootService()
    snodea = NodeAService()
    snodeb = NodeBService()
    sleaf = LeafService()
    server.register_service(sroot)
    server.register_service(snodea)
    server.register_service(snodeb)
    server.register_service(sleaf)

    server.bootstrap_services()

    assert snodea._root is sroot
    assert snodeb._root is sroot
    assert sleaf._nodea is snodea
    assert sleaf._nodeb is snodeb


def test_services_dependencies_missing_dep(server, services):
    _, NodeAService, _, _ = services

    snodea = NodeAService()
    server.register_service(snodea)

    with pytest.raises(RuntimeError) as exc:
        server.bootstrap_services()
    assert exc.value.args[0] == ['Service `NodeAService` required unknown service `RootService`']


def test_services_dependencies_missing_multi_dep(server, services):
    _, NodeAService, NodeBService, LeafService = services

    snodea = NodeAService()
    snodeb = NodeBService()
    sleaf = LeafService()
    server.register_service(snodea)
    server.register_service(snodeb)
    server.register_service(sleaf)

    with pytest.raises(RuntimeError) as exc:
        server.bootstrap_services()
    assert sorted(exc.value.args[0]) == sorted([
        'Service `NodeAService` required unknown service `RootService`',
        'Service `NodeBService` required unknown service `RootService`'
    ])


def test_services_dependencies_child(server, services):
    RootService, NodeAService, _, _ = services

    class ChildService(NodeAService):
        _nodea = service('NodeAService')

    schild = ChildService()
    sroot = RootService()
    snodea = NodeAService()
    server.register_service(schild)
    server.register_service(sroot)
    server.register_service(snodea)

    server.bootstrap_services()
    assert schild._root is sroot
    assert schild._nodea is snodea
