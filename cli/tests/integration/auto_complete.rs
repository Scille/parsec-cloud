// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use predicates::boolean::PredicateBooleanExt;

#[test]
fn gen_bash_completion() {
    crate::assert_cmd_success!("auto-complete", "bash")
        .stdout(predicates::str::is_empty().not())
        .stderr(predicates::str::is_empty());
}
