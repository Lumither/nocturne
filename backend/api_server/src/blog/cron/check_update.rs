pub fn task() {
    // get dir
    // let git_work_dir_path = &env::var("GIT_WORK_DIR").unwrap_or_else(|e| {
    //     if let Ok(default_log_path) = env::var("WORK_DIR") {
    //         if default_log_path.starts_with('~') {
    //             let home_path = home_dir().unwrap_or_else(|| panic!("Cannot find home directory"));
    //             home_path
    //                 .join(default_log_path.strip_prefix("~/").unwrap())
    //                 .into_os_string()
    //                 .into_string()
    //                 .expect(
    //                     "[fatal] failed to parse env `WORK_DIR`, try to use absolute path instead",
    //                 )
    //         } else {
    //             default_log_path
    //         }
    //     } else {
    //         panic!("[fatal] failed to parse env `GIT_WORK_DIR`: {}", e);
    //     }
    // });
    println!("check update");
}
