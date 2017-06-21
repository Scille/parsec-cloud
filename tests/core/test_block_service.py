from effect.testing import perform_sequence

from parsec.core2.block_service import api_block_read, api_block_create, BlockRead, BlockCreate, Block


def test_block_read():
    eff = api_block_read({'id': '1234567890'})
    sequence = [
        (BlockRead('1234567890'), lambda _: Block('1234567890', 'foo')),
    ]
    block = perform_sequence(sequence, eff)
    assert block == {'status': 'ok', 'id': '1234567890', 'content': 'foo'}


def test_block_create():
    eff = api_block_create({'id': '1234567890', 'content': 'foo'})
    sequence = [
        (BlockCreate('1234567890', 'foo'), lambda _: Block('1234567890', 'foo')),
    ]
    block = perform_sequence(sequence, eff)
    assert block == {'status': 'ok', 'id': '1234567890'}
