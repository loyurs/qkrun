use bollard::{image::BuildImageOptions, Docker};
use futures_util::stream::StreamExt;
use std::collections::HashMap;
use tracing::debug;
use tracing::error;
use tracing::info;

//调用docker daemon api进行构建
pub async fn build_images_with_github(git_url: &str, container_name: &str) {
    info!("Starting to build");
    let docker = Docker::connect_with_socket_defaults();
    match docker {
        Ok(docker) => {
            info!("Connect with socket defaults successful");
            let build_options = BuildImageOptions {
                dockerfile: "Dockerfile",
                t: container_name,
                remote: git_url,
                ..Default::default()
            };
            let mut image_build_stream = docker.build_image(build_options, None, None);
            while let Some(msg) = image_build_stream.next().await {
                match msg {
                    Ok(result) => {
                        info!("{:?}", result)
                    }
                    Err(err) => {
                        error!("{:?}", err)
                    }
                }
            }
        }
        Err(err) => {
            error!("connect woth defaults failed");
        }
    }

    //精细化构建所需
    // let mut build_image_args = HashMap::new();
    // build_image_args.insert("dummy", "value");

    // let mut build_image_labels = HashMap::new();
    // build_image_labels.insert("maintainer", "somemaintainer");

    // let build_image_options = BuildImageOptions {
    //     dockerfile: "Dockerfile",
    //     t: "bollard-build-example",
    //     extrahosts: Some("myhost:127.0.0.1"),
    //     remote:
    //         "https://raw.githubusercontent.com/loyurs/qkrun/master/docker/rust_code_server/Dockerfile",
    //     q: false,
    //     nocache: false,
    //     cachefrom: vec![],
    //     pull: true,
    //     rm: true,
    //     forcerm: true,
    //     memory: Some(120000000),
    //     memswap: Some(500000),
    //     cpushares: Some(2),
    //     cpusetcpus: "0-1",
    //     cpuperiod: Some(2000),
    //     cpuquota: Some(1000),
    //     buildargs: build_image_args,
    //     shmsize: Some(1000000),
    //     squash: false,
    //     labels: build_image_labels,
    //     networkmode: "host",
    //     platform: "linux/x86_64",
    // };
}
