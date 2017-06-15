import pytest
import arrow

from parsec.core2.workspace import workspace_factory, Workspace, File, Folder


def test_workspace_factory_load_simple():
    payload = {
        'type': 'folder', 'created': '2017-03-22T12:32:02.993+00',
        'updated': '2017-03-22T12:32:10.872+00', 'children': {}
    }
    wksp = workspace_factory(payload)
    assert isinstance(wksp, Workspace)
    assert isinstance(wksp.created, arrow.Arrow)
    assert isinstance(wksp.updated, arrow.Arrow)
    assert wksp.children == {}


def test_workspace_factory_load_complexe():
    payload = {
        'type': 'folder', 'created': '2017-03-22T12:32:02.993+00',
        'updated': '2017-03-22T12:32:10.872+00', 'children': {
            'foo_file': {
                'type': 'file', 'created': '2017-03-22T12:32:02.993+00',
                'updated': '2017-03-22T12:32:10.872+00'
            },
            'bar_folder': {
                'type': 'folder', 'created': '2017-03-22T12:32:02.993+00',
                'updated': '2017-03-22T12:32:10.872+00', 'children': {
                    'sub_bar_file': {
                        'type': 'file', 'created': '2017-03-22T12:32:02.993+00',
                        'updated': '2017-03-22T12:32:10.872+00'
                    },
                    'sub_bar_folder': {
                        'type': 'folder', 'created': '2017-03-22T12:32:02.993+00',
                        'updated': '2017-03-22T12:32:10.872+00', 'children': {}
                    }
                }
            }
        }
    }
    wksp = workspace_factory(payload)
    assert isinstance(wksp, Workspace)
    assert isinstance(wksp.created, arrow.Arrow)
    assert isinstance(wksp.updated, arrow.Arrow)
    assert sorted(wksp.children.keys()) == sorted(['foo_file', 'bar_folder'])

    foo = wksp.children['foo_file']
    assert isinstance(foo, File)
    assert isinstance(foo.created, arrow.Arrow)
    assert isinstance(foo.updated, arrow.Arrow)

    bar = wksp.children['bar_folder']
    assert isinstance(bar, Folder)
    assert isinstance(bar.created, arrow.Arrow)
    assert isinstance(bar.updated, arrow.Arrow)
    assert sorted(bar.children.keys()) == sorted(['sub_bar_file', 'sub_bar_folder'])

    sub_bar_file = bar.children['sub_bar_file']
    assert isinstance(sub_bar_file, File)
    assert isinstance(sub_bar_file.created, arrow.Arrow)
    assert isinstance(sub_bar_file.updated, arrow.Arrow)

    sub_bar_folder = bar.children['sub_bar_folder']
    assert isinstance(sub_bar_folder, Folder)
    assert isinstance(sub_bar_folder.created, arrow.Arrow)
    assert isinstance(sub_bar_folder.updated, arrow.Arrow)
    assert sub_bar_folder.children == {}
