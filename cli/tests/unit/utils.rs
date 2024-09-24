use libparsec::*;
use uuid::Uuid;

use crate::utils::MINIMAL_SHORT_ID_SIZE;

#[rstest::rstest]
#[case::single_device(
    &["fdf1e92d-809b-4c5b-82e7-59700c1d2aa3".parse::<Uuid>().unwrap().into()],
    MINIMAL_SHORT_ID_SIZE
)]
#[case::first_chart_is_enough(
    &[
        "fdf1e92d-809b-4c5b-82e7-59700c1d2aa3".parse::<Uuid>().unwrap().into(),
        "1fedcc12-0a31-4543-b98f-3fc438efda8f".parse::<Uuid>().unwrap().into(),
    ],
    MINIMAL_SHORT_ID_SIZE
)]
#[case::conflict_until_second_group(
    &[
        "fdf1e92d-809b-4c5b-82e7-59700c1d2aa3".parse::<Uuid>().unwrap().into(),
        "fdf1e92d-0a31-4543-b98f-3fc438efda8f".parse::<Uuid>().unwrap().into(),
    ],
    9
)]
#[case::conflict_until_last_char(
    &[
        "1fedcc12-0a31-4543-b98f-3fc438efda8b".parse::<Uuid>().unwrap().into(),
        "1fedcc12-0a31-4543-b98f-3fc438efda8f".parse::<Uuid>().unwrap().into(),
    ],
    32
)]
#[case::conflict_until_second_char(
    &[
        "1fedcc12-0a31-4543-b98f-3fc438efda8f".parse::<Uuid>().unwrap().into(),
        "10c840c5-b566-491d-85c5-56153d8facef".parse::<Uuid>().unwrap().into(),
        "cb40cb35-76ca-4533-9f2c-c9eb588cda31".parse::<Uuid>().unwrap().into(),
        "eb427366-56af-4137-b4e6-daf652d6bfcb".parse::<Uuid>().unwrap().into(),
    ],
    MINIMAL_SHORT_ID_SIZE // Given that 2 is lower than `MINIMAL_SHORT_ID_SIZE``
)]
#[case::identical_id_short_circuit(
    &[
        "1fedcc12-0a31-4543-b98f-3fc438efda8b".parse::<Uuid>().unwrap().into(),
        "1fedcc12-0a31-4543-b98f-3fc438efda8b".parse::<Uuid>().unwrap().into(),
    ],
    32
)]
fn test_get_minimal_sort_id_len(#[case] ids: &[DeviceID], #[case] expected_size: usize) {
    assert_eq!(
        crate::utils::get_minimal_short_id_size(ids.iter()),
        expected_size
    );
}
