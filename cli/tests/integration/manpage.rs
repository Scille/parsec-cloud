use libparsec::{tmp_path, TmpPath};
use parsec_cli::commands::man_page::Mode;

#[rstest::rstest]
#[case::all_in_one(Mode::AllInOne)]
#[case::separate(Mode::Separate)]
#[tokio::test]
async fn gen_man_page(#[case] mode: Mode, tmp_path: TmpPath) {
    let out_path = tmp_path.join("man");

    if mode == Mode::Separate {
        tokio::fs::create_dir(&out_path).await.unwrap();
    }
    crate::assert_cmd_success!(
        "man-page",
        "--mode",
        &mode.to_string(),
        &out_path.to_string_lossy()
    )
    .stdout(predicates::str::is_empty())
    .stderr(predicates::str::is_empty());

    let metadata = tokio::fs::metadata(&out_path).await.unwrap();
    match mode {
        Mode::AllInOne => {
            assert!(metadata.is_file());
            #[cfg(target_family = "unix")]
            {
                use std::os::unix::fs::MetadataExt;
                assert!(metadata.size() > 0);
            }
        }
        Mode::Separate => {
            assert!(metadata.is_dir());
            let mut entries = tokio::fs::read_dir(out_path).await.unwrap();
            let mut count = 0_usize;
            while let Ok(Some(_)) = entries.next_entry().await {
                count += 1;
            }
            assert!(count > 1);
        }
    }
}
