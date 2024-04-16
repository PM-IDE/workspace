use std::fs;
use std::path::PathBuf;

pub fn execute_test_with_gold<T>(gold_file_path: PathBuf, test_func: T)
where
    T: FnOnce() -> String,
{
    let gold_file_dir = gold_file_path.parent().unwrap();
    if !gold_file_dir.exists() {
        fs::create_dir_all(gold_file_dir).ok();
    }

    let test_value = test_func();

    let write_tmp = || {
        let file_name = gold_file_path.file_name().unwrap().to_str().unwrap();
        let tmp_file_path = gold_file_dir.join(file_name.to_owned() + ".tmp");
        fs::write(&tmp_file_path, &test_value).ok();
    };

    if gold_file_path.exists() {
        let gold_content = String::from_utf8(fs::read(&gold_file_path).ok().unwrap()).ok().unwrap();
        if gold_content != test_value {
            write_tmp();
            panic!("Gold and test values are not equal for {}", gold_file_path.display());
        }

        return;
    }

    write_tmp();
    panic!("There was no gold for {}", gold_file_path.display());
}
