use std::io::Write;
use std::process::{Command, Stdio};

///直接通过内置dockerfile构造
pub fn build_rust_dev_image_with_extensions(image_name_and_tag:&str) {
    let dockerfile = r#"FROM registry.cn-hangzhou.aliyuncs.com/clouddevs/vscode-extensions:rust AS builder
FROM registry.cn-hangzhou.aliyuncs.com/clouddevs/code-server:ubuntu20-ssh
COPY --from=builder  /opt/extensions /config/extensions
RUN mkdir -p /root/.vscode-server && ln -s /config/extensions /root/.vscode-server/extensions"#;
    let mut child = Command::new("docker")
    .env("DOCKER_BUILDKIT", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .args(&["build","-","-t",image_name_and_tag])
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(dockerfile.as_bytes())
            .expect("Failed to write to stdin");
    });
    let output = child.wait_with_output().expect("Failed to read stdout");
    println!("{:?}",String::from_utf8_lossy(&output.stdout));
    println!("****Done****");
}


///通过github文件进行构造
pub fn git2build(git_urls:&str, image_name_and_tag:&str) {
    println!("{:?}",git_urls);
    Command::new("docker")
    .args(&["build",git_urls,"-t",image_name_and_tag])
    .spawn()
    .expect("Build use git error");
}
