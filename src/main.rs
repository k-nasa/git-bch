use std::process;

fn main() {
    let git_branch_process = process::Command::new("git")
        .arg("branch")
        .stdout(process::Stdio::piped())
        .spawn()
        .expect("Failed run git-branch command");

    let output_branch_raw = git_branch_process.wait_with_output().unwrap().stdout;
    let mut output_branch: Vec<String> = String::from_utf8(output_branch_raw)
        .unwrap()
        .split('\n')
        .map(|branch| branch.trim().to_string())
        .filter(|branch| !branch.is_empty())
        .collect();
    output_branch.sort();

    println!("{:?}", output_branch);
}

struct BranchTree {
    branches: Vec<String>,
    node: Vec<BranchNode>,
}

struct BranchNode {
    name: String,
    branches: Vec<String>,
    node: Vec<BranchNode>,
}
