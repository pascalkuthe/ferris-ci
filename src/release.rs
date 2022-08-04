// pub fn creae_release_pr(token: String) -> anyhow::Result<()> {
//     let api = pulls::new(&auth);
//     let params = PullsListParams::new().state("OPEN");
//     api.list("pascalkuthe", "openvaf", Some(params))?;

use std::fs;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::str::Lines;

use anyhow::bail;
use chrono::Utc;
//     Ok(())
// }
use serde::Deserialize;
use xshell::{cmd, Shell};

fn find_release_pr<'a>(project: &str, pr_list: &'a [PullRequest]) -> Option<&'a PullRequest> {
    pr_list
        .iter()
        .find(|pr| pr.title_contains(project) && pr.has_label("autorelease: pending"))
}

fn collect_changelog(lines: Lines) -> String {
    let mut changelog = String::new();
    for line in lines {
        if line.trim().starts_with("##") {
            break;
        }

        if !line.trim().is_empty() {
            changelog.push_str(line);
            changelog.push('\n')
        }
    }
    changelog
}

fn find_unreleased_section(lines: &mut Lines) -> Option<Range<usize>> {
    let mut line_start = 0;
    let line_end = loop {
        let line = lines.next()?;
        if let Some(rem) = line.trim_start().strip_prefix("##") {
            if rem.trim_start() == "[unreleased]" {
                break line_start + line.len();
            } else {
                return None;
            }
        }
        line_start += line.len() + 1;
    };

    Some(line_start..line_end)
}

fn update_changelog(content: &mut String, new_version: &str) -> Option<String> {
    let mut lines = content.lines();
    let range = find_unreleased_section(&mut lines)?;
    let changelog = collect_changelog(lines);

    let date = Utc::now().format("%Y-%m-%d");
    let new_header = format!("## {new_version} - {date}");
    content.replace_range(range, &new_header);

    Some(changelog)
}

fn find_latest_version(sh: &Shell, project: &str) -> anyhow::Result<Option<String>> {
    let tags = cmd!(sh, "git tag -l --sort=-v:refname {project}-*").read()?;
    let latest_tag = tags.lines().next();
    let version = latest_tag
        .and_then(|tag| tag.strip_prefix(&format!("{project}-")))
        .map(str::to_owned);
    Ok(version)
}

fn create_release_commit(
    sh: &Shell,
    project: &str,
    old_version: &str,
    new_version: &str,
    files: &[PathBuf],
) -> anyhow::Result<Option<String>> {
    cmd!(sh, "git checkout autorelease-{project}").run()?;
    cmd!(sh, "git reset --hard origin/master").run()?;

    // update changelog
    let changelog_file = format!("CHANGELOG_{}.md", project.to_uppercase());
    let changelog_file = sh.current_dir().join(changelog_file);
    let mut content = fs::read_to_string(&changelog_file)?.replace("\r\n", "\n");
    let changelog = update_changelog(&mut content, new_version);
    if changelog.is_none() {
        return Ok(None);
    }
    fs::write(changelog_file, content)?;

    replace_versions(sh, old_version, new_version, false, files)?;

    let commit_message = format!("chore({project}): release v{new_version}");
    cmd!(sh, "git add --all").run()?;
    cmd!(sh, "git commit -m {commit_message}").run()?;
    cmd!(sh, "git push --force --set-upstream origin/{new_version}").run()?;
    Ok(changelog)
}

fn create_release_cycle_commit(
    sh: &Shell,
    project: &str,
    old_version: &str,
    new_version: &str,
    files: &[PathBuf],
) -> anyhow::Result<()> {
    replace_versions(sh, old_version, new_version, true, files)?;

    let new_version = semvar_dev_version(new_version);
    let commit_message = format!("chore({project}): start development cycle v{new_version}");
    cmd!(sh, "git add --all").run()?;
    cmd!(sh, "git commit -m {commit_message}").run()?;
    cmd!(sh, "git push --force --set-upstream origin/{new_version}").run()?;
    Ok(())
}

fn semvar_dev_version(version: &str) -> String {
    let (major, rem) = version.split_once('.').unwrap();
    let (minor, rem) = rem.split_once('.').unwrap();
    let (patch, _) = rem.split_once('.').unwrap();
    let patch: u32 = patch.parse().unwrap();
    format!("{major}.{minor}.{}-dev", patch + 1)
}

fn python_dev_version(version: &str) -> String {
    let (major, rem) = version.split_once('.').unwrap();
    let (minor, rem) = rem.split_once('.').unwrap();
    let (patch, _) = rem.split_once('.').unwrap();
    let patch: u32 = patch.parse().unwrap();
    format!("{major}.{minor}.{}dev", patch + 1)
}

fn dev_version(version: &str, file: &Path) -> anyhow::Result<String> {
    let version = match file.file_name().and_then(|ext| ext.to_str()) {
        Some("Cargo.toml" | "Cargo.lock") => semvar_dev_version(&version),
        Some("setup.py") => python_dev_version(&version),
        _ => bail!("Unkown faile type {}", file.display()),
    };
    Ok(version)
}

fn replace_versions(
    sh: &Shell,
    old_version: &str,
    new_version: &str,
    to_devel: bool,
    files: &[PathBuf],
) -> anyhow::Result<()> {
    for file in files {
        let old_version = dev_version(old_version, file)?;
        let new_version = if to_devel {
            dev_version(new_version, file)?
        } else {
            cmd!(sh, "git checkout HEAD^ -- {file}").run()?;
            new_version.to_owned()
        };

        let mut content = fs::read_to_string(file)?;
        content.replace(&old_version, &new_version);
        fs::write(file, &content);
    }

    Ok(())
}

fn list_pull_requests(token: &str) -> anyhow::Result<Vec<PullRequest>> {
    let response = call_github_api(
        token,
        "pulls",
        &[],
        &[("base", "master"), ("per_page", "100")],
    )?
    .into_string()?;
    let res = serde_json::from_str(&response)?;
    Ok(res)
}

const GITHUB_API: &str = "https://api.github.com/repos/pascalkuthe/openvaf";
fn call_github_api(
    token: &str,
    endpoint: &str,
    headers: &[(&str, &str)],
    queries: &[(&str, &str)],
) -> anyhow::Result<ureq::Response> {
    let url = format!("{GITHUB_API}/{endpoint}");
    let mut request = ureq::get(&url)
        .set("Accept", "application/vnd.github+json")
        .set("Authorization", token);
    for (param, value) in headers.iter() {
        request = request.set(param, value)
    }
    for (param, value) in queries.iter() {
        request = request.query(param, value);
    }
    let res = request.call()?;
    Ok(res)
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct PullRequest {
    pub url: String,
    pub id: u64,
    pub title: Option<String>,
    pub labels: Option<Vec<Label>>,
}

impl PullRequest {
    pub fn title_contains(&self, substr: &str) -> bool {
        if let Some(title) = &self.title {
            title.contains(substr)
        } else {
            false
        }
    }
    pub fn has_label(&self, label: &str) -> bool {
        if let Some(labels) = &self.labels {
            labels.iter().any(|it| it.name == label)
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct Label {
    pub id: u64,
    pub name: String,
}
