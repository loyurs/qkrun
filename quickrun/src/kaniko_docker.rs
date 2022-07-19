use serde_json::json;
use std::process::Command;
use std::process::Stdio;
use tracing::info;
const IMAGE: &str = "registry.cn-hangzhou.aliyuncs.com/clouddevs/kanico:latest";
///use kaniko to build with git
/// 利用git及其子目录进行构建，docker
pub fn build_kaniko() {
    let workspace_map = "/home:/workspace";
    let config_json_map = "/home/config.json:/kaniko/.docker/config.json:ro";
    let git_url = "git://github.com/loyurs/qkrun.git#refs/heads/master";
    let git_subfolder = "dockerfiles/test/";
    let dest_image = "ccr.ccs.tencentyun.com/tctd/yuxin:love";
    Command::new("docker")
        .args(&[
            "run",
            "-ti",
            "--rm",
            "-v",
            workspace_map,
            "-v",
            config_json_map,
            IMAGE,
            "--context",
            git_url,
            "--context-sub-path",
            git_subfolder,
            "--dockerfile",
            "Dockerfile",
            "--destination",
            dest_image,
        ])
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .unwrap();
    // std::fs::remove_file("/home/config.json").unwrap();
}

///use kaniko to build with context
pub fn generaste_base64_secret(user: &str, password: &str, url: &str) {
    let based64 = base64::encode(format!("{}:{}", user, password));
    let pp = json!({
        "auths": {
            url: {
                "auth": based64
            }
        }
    });
    std::fs::write("/home/config.json", pp.to_string().as_bytes()).unwrap();
    info!("成功生成docker push key /home/config.json");
    // println!("{}", pp);
}

///利用本地dockerfile进行构建

pub fn build_with_local_dockerfile(input_str: &str) {
    let echo_child = Command::new("echo")
        .arg(input_str)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start echo process");
    let echo_out = echo_child.stdout.expect("Failed to open echo stdout");
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg("echo")
        .stdin(Stdio::from(echo_out))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

#[test]
fn kakaka() {
    build_with_local_dockerfile("input_dockerfiles");
}
