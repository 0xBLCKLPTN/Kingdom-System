//!
//!    Filename: tests.rs
//!    Description: Module with all functions test.
//!    Author: Daniil (0xJanus) Ermolaev
//!

use crate::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn check_on_file_exists_test() {
        let k = check_on_file_exists(&PathBuf::from("./main.rs")).await;
        assert_eq!(k, true);
    }
    
    #[tokio::test]
    pub async fn pathbuf_to_string_test() {
        let result = pathbuf_to_string(&PathBuf::from("./main.rs")).await.unwrap();
        assert_eq!("./main.rs", result.as_str());
    }

    #[tokio::test]
    pub async fn listdir_test() {
        let files = listdir(&PathBuf::from("../../Tests/alicedb_test_folder")).await.unwrap();
        assert_eq!(
            vec![
                PathBuf::from(
                    "../../Tests/alicedb_test_folder/file1.test"
                ),
                PathBuf::from(
                    "../../Tests/alicedb_test_folder/file2.test"
                )
            ],
            files
        );
    }
    
    #[tokio::test]
    pub async fn create_dir_test() {
        let res = create_dir(&PathBuf::from("./test_dir")).await.unwrap();
        assert_eq!((), res);
    }

    #[tokio::test]
    pub async fn delete_dir_test() {
        let res = delete_dir(&PathBuf::from("./test_dir")).await.unwrap();
        assert_eq!((), res);
    }

    #[tokio::test]
    pub async fn create_file_test() {
        let k = create_file(&PathBuf::from("./testfile")).await.unwrap();
        assert_eq!((), k);
    }

    #[tokio::test]
    pub async fn simple_write_test() {
        create_file(&PathBuf::from("./testfile2")).await.unwrap();
        let k = simple_write(&PathBuf::from("./testfile2"), "data").await.unwrap();
        delete_file(&PathBuf::from("./testfile2")).await.unwrap();
        assert_eq!((), k);
    }

    #[tokio::test]
    pub async fn delete_file_test() {
        let k = delete_file(&PathBuf::from("./testfile")).await.unwrap();
        assert_eq!((), k);
    }
    #[tokio::test]
    pub async fn read_file_test() {
        create_file(&PathBuf::from("./testfile3")).await.unwrap();
        let k = simple_write(&PathBuf::from("./testfile3"), "data").await.unwrap();
        let out = read_file(&PathBuf::from("./testfile3")).await.unwrap();
        delete_file(&PathBuf::from("./testfile3")).await.unwrap();
        assert_eq!(out, "data\n");
    }
}