use libparsec::{tmp_path, TmpPath};
use parsec_cli::ui::Ui;

use crate::{bootstrap_cli_test, test_ui};

#[rstest::rstest]
#[tokio::test]
async fn list_devices(tmp_path: TmpPath, test_ui: &Ui) {
    bootstrap_cli_test(test_ui, &tmp_path).await.unwrap();

    let path = tmp_path.join("config/parsec3/libparsec");
    let path_str = path.to_string_lossy();

    crate::assert_cmd_success!("device", "list").stderr(predicates::str::contains(format!(
        "Found 4 device(s) in {path_str}"
    )));
}
