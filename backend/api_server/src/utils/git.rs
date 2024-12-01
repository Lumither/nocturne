/// git merge
/// based on https://github.com/rust-lang/git2-rs/blob/master/examples/pull.rs
/// https://github.com/rust-lang/git2-rs/commit/f3b87baed1e33d6c2d94fe1fa6aa6503a071d837
///
use git2::{
    build, AnnotatedCommit, AutotagOption, Delta, FetchOptions, Reference, Remote, Repository,
};
use std::path::PathBuf;
use tracing::{error, warn};


#[derive(Debug, Eq, PartialEq)]
pub struct FileDelta {
    pub old_path: Option<PathBuf>,
    pub new_path: Option<PathBuf>,
    pub status: Delta,
}

fn do_fetch<'a>(
    repo: &'a Repository,
    refs: &[&str],
    remote: &'a mut Remote,
) -> Result<AnnotatedCommit<'a>, git2::Error> {
    let mut fetch_option = FetchOptions::new();

    // Always fetch all tags.
    // Perform a download and also update tips
    fetch_option.download_tags(AutotagOption::All);
    remote.fetch(refs, Some(&mut fetch_option), None)?;

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    repo.reference_to_annotated_commit(&fetch_head)
}

fn fast_forward(
    repo: &Repository,
    lb: &mut Reference,
    rc: &AnnotatedCommit,
) -> Result<(), git2::Error> {
    let name = match lb.name() {
        Some(s) => s.to_string(),
        None => String::from_utf8_lossy(lb.name_bytes()).to_string(),
    };
    let msg = format!("fast-forward({}) -> {}", name, rc.id());
    // println!("{}", msg);
    lb.set_target(rc.id(), &msg)?;
    repo.set_head(&name)?;
    repo.checkout_head(Some(
        build::CheckoutBuilder::default()
            // For some reason, the force is required to make the working directory actually get updated
            // I suspect we should be adding some logic to handle dirty working directory states,
            // but this is just an example so maybe not.
            .force(),
    ))?;
    Ok(())
}

fn normal_merge(
    repo: &Repository,
    local: &AnnotatedCommit,
    remote: &AnnotatedCommit,
) -> Result<(), git2::Error> {
    let local_tree = repo.find_commit(local.id())?.tree()?;
    let remote_tree = repo.find_commit(remote.id())?.tree()?;
    let ancestor = repo
        .find_commit(repo.merge_base(local.id(), remote.id())?)?
        .tree()?;
    let mut idx = repo.merge_trees(&ancestor, &local_tree, &remote_tree, None)?;

    if idx.has_conflicts() {
        warn!("merge conflicts detected");
        repo.checkout_index(Some(&mut idx), None)?;
        return Ok(());
    }
    let result_tree = repo.find_tree(idx.write_tree_to(repo)?)?;
    // now create the merge commit
    let msg = format!("merge({}) -> {}", remote.id(), local.id());
    let sig = repo.signature()?;
    let local_commit = repo.find_commit(local.id())?;
    let remote_commit = repo.find_commit(remote.id())?;

    // Do our merge commit and set the current branch head to that commit.
    let _merge_commit = repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &msg,
        &result_tree,
        &[&local_commit, &remote_commit],
    )?;
    // Set working tree to match head.
    repo.checkout_head(None)?;
    Ok(())
}

fn do_merge<'a>(
    repo: &'a Repository,
    remote_branch: &str,
    fetch_commit: AnnotatedCommit<'a>,
) -> Result<Vec<FileDelta>, git2::Error> {
    // 1. do a merge analysis
    let analysis = repo.merge_analysis(&[&fetch_commit])?;

    let head_commit = repo.reference_to_annotated_commit(&repo.head()?)?;

    // 2. Do the appropriate merge
    if analysis.0.is_fast_forward() {
        // println!("Doing a fast-forward");
        // do a fast-forward
        let ref_name = format!("refs/heads/{}", remote_branch);
        match repo.find_reference(&ref_name) {
            Ok(mut r) => {
                fast_forward(repo, &mut r, &fetch_commit)?;
            }
            Err(_) => {
                // The branch doesn't exist, so set the reference to the
                // commit directly. Usually this is because you are pulling
                // into an empty repository.
                repo.reference(
                    &ref_name,
                    fetch_commit.id(),
                    true,
                    &format!("({}) -> {}", remote_branch, fetch_commit.id()),
                )?;
                repo.set_head(&ref_name)?;
                repo.checkout_head(Some(
                    build::CheckoutBuilder::default()
                        .allow_conflicts(true)
                        .conflict_style_merge(true)
                        .force(),
                ))?;
            }
        };
    } else if analysis.0.is_normal() {
        // do a normal merge
        normal_merge(repo, &head_commit, &fetch_commit)?;
    }
    // else {
    //     // already up-to-date
    //     println!("Nothing to do...");
    // }

    calculate_diff(repo, &head_commit, &fetch_commit)
}

fn calculate_diff(
    repo: &Repository,
    local: &AnnotatedCommit,
    remote: &AnnotatedCommit,
) -> Result<Vec<FileDelta>, git2::Error> {
    let local_tree = repo.find_commit(local.id())?.tree()?;
    let remote_tree = repo.find_commit(remote.id())?.tree()?;
    let diff = repo.diff_tree_to_tree(Some(&local_tree), Some(&remote_tree), None)?;

    let mut updated_files: Vec<FileDelta> = Vec::new();

    if let Err(e) = diff.foreach(
        &mut |d, _| {
            // let delta = d.status();
            //
            // let n_path = d.new_file().path();
            // let o_path = d.old_file().path();
            //
            // if let Some(path) = n_path {
            //     updated_files.insert(path.to_owned());
            // }
            // if let Some(path) = o_path {
            //     updated_files.insert(path.to_owned());
            // }
            updated_files.push(FileDelta {
                old_path: d.old_file().path().map(PathBuf::from),
                new_path: d.new_file().path().map(PathBuf::from),
                status: d.status(),
            });
            true // Continue
        },
        None,
        None,
        None,
    ) {
        error!("error at calculating diff: {:?}", e);
    }

    Ok(updated_files.into_iter().collect())
}

pub fn sync(
    remote_name: &str,
    remote_branch: &str,
    repo: &Repository,
) -> Result<Vec<FileDelta>, git2::Error> {
    let mut remote = repo.find_remote(remote_name)?;
    let fetch_commit = do_fetch(repo, &[remote_branch], &mut remote)?;
    do_merge(repo, remote_branch, fetch_commit)
}
