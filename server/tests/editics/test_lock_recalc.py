# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from __future__ import annotations

from typing import Literal

import pytest

from parsec._parsec import DeviceID
from parsec.components.editics import (
    EditicsDocumentSession,
    LockRecord,
    _recalculate_locks,
)
from parsec.editics_protocol import (
    EditicsExcelRangeOrObjectID,
    EditicsExcelRecalcIndex,
    EditicsExcelRecalcIndexElement,
    EditicsExcelSheetRecalcIndex,
    EditicsParticipantIdentity,
)

SHEET_ID = "6"

ALICE: EditicsParticipantIdentity = (
    DeviceID.from_hex("de10a11cec0010000000000000000000"),
    1,
)
BOB: EditicsParticipantIdentity = (
    DeviceID.from_hex("de10808c001000000000000000000000"),
    2,
)


def _recalc_index(
    *,
    recalc_type: Literal[1, 2],
    position: int,
    count: int = 1,
    is_save_index: bool = False,
) -> EditicsExcelRecalcIndex:
    return {
        SHEET_ID: EditicsExcelSheetRecalcIndex(
            _arrElements=[
                EditicsExcelRecalcIndexElement(
                    _recalcType=recalc_type,
                    _position=position,
                    _count=count,
                    m_bIsSaveIndex=is_save_index,
                )
            ]
        )
    }


def _range_lock(
    *,
    holder: EditicsParticipantIdentity,
    c1: int,
    c2: int,
    r1: int,
    r2: int,
    sheet_id: str = SHEET_ID,
    sub_type: int | None = 0,
) -> LockRecord:
    return LockRecord(
        time=1,
        holder=holder,
        block={
            "guid": f"{sheet_id}-{c1}-{r1}-{c2}-{r2}",
            "type": 1,  # `c_oAscLockTypeElem.Range`
            "subType": sub_type,
            "sheetId": sheet_id,
            "rangeOrObjectId": {"c1": c1, "c2": c2, "r1": r1, "r2": r2},
        },
    )


def _session_with_lock(lock: LockRecord, key: str = "lock") -> EditicsDocumentSession:
    session = EditicsDocumentSession()
    session.region_locks[key] = lock
    return session


def _coords(session: EditicsDocumentSession, key: str = "lock") -> EditicsExcelRangeOrObjectID:
    block = session.region_locks[key].block
    assert isinstance(block, dict) and "rangeOrObjectId" in block
    return block["rangeOrObjectId"]


# --- Insertion ---------------------------------------------------------------


@pytest.mark.parametrize(
    ("locked_c1", "expected_c1"),
    [
        # Inserting at A shifts everything at or after A to the right...
        (0, 1),
        (1, 2),
        (5, 6),
    ],
)
def test_insert_column_shifts_locks_at_or_after_position(locked_c1: int, expected_c1: int) -> None:
    session = _session_with_lock(_range_lock(holder=ALICE, c1=locked_c1, c2=locked_c1, r1=0, r2=0))

    _recalculate_locks(
        session, BOB, index_cols=_recalc_index(recalc_type=1, position=0), index_rows=None
    )

    assert _coords(session)["c1"] == expected_c1
    assert _coords(session)["c2"] == expected_c1


def test_insert_rows_only_shifts_rows() -> None:
    session = _session_with_lock(_range_lock(holder=ALICE, c1=3, c2=3, r1=2, r2=2))

    _recalculate_locks(
        session, BOB, index_cols=None, index_rows=_recalc_index(recalc_type=1, position=2)
    )

    # Only the row axis is touched
    assert _coords(session) == {"c1": 3, "c2": 3, "r1": 3, "r2": 3}


def test_insert_multi_count_shifts_by_count() -> None:
    session = _session_with_lock(_range_lock(holder=ALICE, c1=4, c2=4, r1=0, r2=0))

    _recalculate_locks(
        session,
        BOB,
        index_cols=_recalc_index(recalc_type=1, position=1, count=3),
        index_rows=None,
    )

    assert _coords(session)["c1"] == 7


# --- Deletion ----------------------------------------------------------------


@pytest.mark.parametrize(
    ("locked_c1", "expected_c1"),
    [
        # Deleting column A: a lock on A collapses onto `position - 1`, i.e. -1
        # (this is what the OnlyOffice server's `_addRecalcIndex` + `getLockMe2`
        # pipeline computes: `max(position - 1, coord - count)`).
        (0, -1),
        # A lock past the deleted range shifts left by one
        (1, 0),
        (5, 4),
    ],
)
def test_delete_column_at_first_position(locked_c1: int, expected_c1: int) -> None:
    session = _session_with_lock(_range_lock(holder=ALICE, c1=locked_c1, c2=locked_c1, r1=0, r2=0))

    _recalculate_locks(
        session, BOB, index_cols=_recalc_index(recalc_type=2, position=0), index_rows=None
    )

    coords = _coords(session)
    assert coords["c1"] == expected_c1
    assert coords["c2"] == expected_c1


@pytest.mark.parametrize(
    ("locked_c1", "expected_c1"),
    [
        # Locked before the deleted range: untouched
        (0, 0),
        (1, 1),
        # Locked on the first deleted column: collapses onto `position - 1`
        (2, 1),
        # Locked inside the deleted range: collapses onto `position - 1`
        (3, 1),
        (4, 1),
        # Locked past the deleted range: shifts left by `count`
        (5, 2),
        (6, 3),
    ],
)
def test_delete_column_range(locked_c1: int, expected_c1: int) -> None:
    # Delete columns C, D, E (0-based positions 2, 3, 4)
    session = _session_with_lock(_range_lock(holder=ALICE, c1=locked_c1, c2=locked_c1, r1=0, r2=0))

    _recalculate_locks(
        session,
        BOB,
        index_cols=_recalc_index(recalc_type=2, position=2, count=3),
        index_rows=None,
    )

    assert _coords(session)["c1"] == expected_c1


def test_delete_first_columns_collapses_onto_position_minus_one() -> None:
    # Delete the two first columns while a lock sits on the first one: like
    # the OnlyOffice server, the lock collapses onto `position - 1` (= -1
    # here, a quirk of the reference implementation we reproduce as-is).
    session = _session_with_lock(_range_lock(holder=ALICE, c1=0, c2=0, r1=0, r2=0))

    _recalculate_locks(
        session,
        BOB,
        index_cols=_recalc_index(recalc_type=2, position=0, count=2),
        index_rows=None,
    )

    coords = _coords(session)
    assert coords["c1"] == -1
    assert coords["c2"] == -1


def test_delete_rows_only_shifts_rows() -> None:
    session = _session_with_lock(_range_lock(holder=ALICE, c1=3, c2=3, r1=0, r2=0))

    _recalculate_locks(
        session, BOB, index_cols=None, index_rows=_recalc_index(recalc_type=2, position=0)
    )

    # The lock on the first row collapses onto `position - 1` (reference quirk)
    assert _coords(session) == {"c1": 3, "c2": 3, "r1": -1, "r2": -1}


# --- What must be left alone -------------------------------------------------


def test_save_index_markers_are_skipped() -> None:
    # A client re-emits the structural change it just applied as a save-index
    # marker: it is an acknowledgement, not a new geometry change.
    session = _session_with_lock(_range_lock(holder=ALICE, c1=2, c2=2, r1=0, r2=0))

    _recalculate_locks(
        session,
        BOB,
        index_cols=_recalc_index(recalc_type=1, position=0, is_save_index=True),
        index_rows=None,
    )

    assert _coords(session)["c1"] == 2


def test_saver_own_locks_are_not_recalculated() -> None:
    session = _session_with_lock(_range_lock(holder=BOB, c1=2, c2=2, r1=0, r2=0))

    _recalculate_locks(
        session, BOB, index_cols=_recalc_index(recalc_type=1, position=0), index_rows=None
    )

    assert _coords(session)["c1"] == 2


def test_locks_of_another_sheet_are_not_recalculated() -> None:
    session = _session_with_lock(_range_lock(holder=ALICE, c1=2, c2=2, r1=0, r2=0, sheet_id="42"))

    _recalculate_locks(
        session, BOB, index_cols=_recalc_index(recalc_type=1, position=0), index_rows=None
    )

    assert _coords(session)["c1"] == 2


def test_structural_lock_sub_types_are_not_recalculated() -> None:
    # `InsertColumns` (2) / `InsertRows` (4) locks are the geometry-change
    # markers themselves, not ranges to shift.
    for sub_type in (2, 4):
        session = _session_with_lock(
            _range_lock(holder=ALICE, c1=2, c2=2, r1=0, r2=0, sub_type=sub_type)
        )

        _recalculate_locks(
            session, BOB, index_cols=_recalc_index(recalc_type=1, position=0), index_rows=None
        )

        assert _coords(session)["c1"] == 2


def test_non_range_blocks_are_not_recalculated() -> None:
    # e.g. the document editor locks by block id (a plain string)
    session = EditicsDocumentSession()
    session.region_locks["block"] = LockRecord(time=1, holder=ALICE, block="some-block-id")

    _recalculate_locks(
        session, BOB, index_cols=_recalc_index(recalc_type=1, position=0), index_rows=None
    )

    assert session.region_locks["block"].block == "some-block-id"


def test_no_recalc_info_is_a_noop() -> None:
    session = _session_with_lock(_range_lock(holder=ALICE, c1=2, c2=2, r1=2, r2=2))

    _recalculate_locks(session, BOB, index_cols=None, index_rows=None)

    assert _coords(session) == {"c1": 2, "c2": 2, "r1": 2, "r2": 2}


def test_several_changes_are_applied_in_order() -> None:
    # Two consecutive insertions at the same position: the second one applies
    # on top of the first, so the lock ends up shifted by 2.
    session = _session_with_lock(_range_lock(holder=ALICE, c1=3, c2=3, r1=0, r2=0))

    index_cols: EditicsExcelRecalcIndex = {
        SHEET_ID: EditicsExcelSheetRecalcIndex(
            _arrElements=[
                EditicsExcelRecalcIndexElement(
                    _recalcType=1, _position=0, _count=1, m_bIsSaveIndex=False
                ),
                EditicsExcelRecalcIndexElement(
                    _recalcType=1, _position=0, _count=1, m_bIsSaveIndex=False
                ),
            ]
        )
    }

    _recalculate_locks(session, BOB, index_cols=index_cols, index_rows=None)

    assert _coords(session)["c1"] == 5
