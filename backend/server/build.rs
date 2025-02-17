use chrono::Utc;
use git2::Repository;

fn main() {
    let repo = Repository::open("../..").expect("failed to open repository");
    let commit = repo
        .head()
        .expect("failed to get HEAD")
        .peel_to_commit()
        .expect("failed to get HEAD commit");
    let hash = commit.id().to_string();
    let utc_date = Utc::now().date_naive().format("%Y%m%d").to_string();
    let build_id = format!("{}@{}", utc_date, &hash[..8]);
    println!("cargo:rustc-env=BUILD_ID={}", build_id);
}
