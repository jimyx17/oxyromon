use super::super::database::*;
use super::super::import_dats;
use super::super::import_roms;
use super::*;
use std::env;
use std::path::PathBuf;
use tempfile::{NamedTempFile, TempDir};
use tokio::fs;

#[tokio::test]
async fn test() {
    if dolphin::get_version().await.is_err() {
        return;
    }

    // given
    let _guard = MUTEX.lock().await;

    let test_directory = Path::new("tests");
    env::set_var(
        "PATH",
        format!(
            "{}:{}",
            test_directory.as_os_str().to_str().unwrap(),
            env::var("PATH").unwrap()
        ),
    );
    let progress_bar = ProgressBar::hidden();

    let db_file = NamedTempFile::new().unwrap();
    let pool = establish_connection(db_file.path().to_str().unwrap()).await;
    let mut connection = pool.acquire().await.unwrap();

    let rom_directory = TempDir::new_in(&test_directory).unwrap();
    set_rom_directory(PathBuf::from(rom_directory.path()));
    let tmp_directory = TempDir::new_in(&test_directory).unwrap();
    let tmp_directory = set_tmp_directory(PathBuf::from(tmp_directory.path()));

    let matches = import_dats::subcommand()
        .get_matches_from(&["import-dats", "tests/Test System (20230618) (RVZ).dat"]);
    import_dats::main(&mut connection, &matches, &progress_bar)
        .await
        .unwrap();

    let romfile_path = tmp_directory.join("Test Game (USA).rvz");
    fs::copy(test_directory.join("Test Game (USA).rvz"), &romfile_path)
        .await
        .unwrap();

    let system = find_systems(&mut connection).await.remove(0);

    let matches = import_roms::subcommand()
        .get_matches_from(&["import-roms", romfile_path.as_os_str().to_str().unwrap()]);
    import_roms::main(&mut connection, &matches, &progress_bar)
        .await
        .unwrap();

    // when
    check_system(
        &mut connection,
        &progress_bar,
        &system,
        &None,
        false,
        &HashAlgorithm::Crc,
    )
    .await
    .unwrap();

    // then
    let mut romfiles = find_romfiles(&mut connection).await;
    assert_eq!(romfiles.len(), 1);

    let romfile = romfiles.remove(0);
    assert!(!romfile.path.contains("/Trash/"));
    assert!(Path::new(&romfile.path).is_file());
}
