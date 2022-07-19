use std::path::{Path, PathBuf};

#[warn(unused)]
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "")]
#[clap(author = "loyu loyurs@163.com")]
#[clap(version = "0.0.1")]
#[clap(about = "make a docker vscode-server", long_about = None)]
// quickrun build/start {}
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 利用giturl构建镜像并自动上传
    Build {
        #[clap(value_parser, long)]
        name: String,
        #[clap(value_parser, long)]
        git: String,
        #[clap(value_parser, long)]
        subpath: String,
        #[clap(value_parser, long, short)]
        user_registry: String,
        #[clap(value_parser, long)]
        passwd_registry: String,
        #[clap(value_parser, long)]
        registry_api: String,
        #[clap(value_parser, long)]
        image_name: String,
    },
    ///start a statefulset by use kubectl
    Start {
        ///利用此项进行删除
        #[clap(value_parser, long)]
        delete: Option<String>,
        ///k8s pv name, pvc name is pvname-pvc
        #[clap(value_parser, long)]
        pv: String,
        #[clap(value_parser, long)]
        pvc: String,
        ///host_path local mount point ,eg: /home
        #[clap(value_parser, long, short)]
        volume: PathBuf,
        ///container name
        #[clap(value_parser, long, short)]
        stsname: String,
        ///vscode-web password
        #[clap(value_parser, long, short)]
        passwd: String,
    },
}
use anyhow::Error;

use crate::{
    k8sapi::apply_delete,
    templates::models::{self, new_docker_registry, new_kaniko_build, new_vscode_server_pod},
};
pub async fn cli_run() -> Result<(), Error> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Start {
            ///PersistentVolume name
            pv,
            ///PersistenVolumeClaim name
            pvc,
            ///mounted disk path
            volume,
            ///statefulset name
            stsname,
            ///container password
            passwd,
            ///optional if none .create if not none delete
            delete,
        } => {
            let yaml = new_vscode_server_pod(
                &models::VSCODE_SERVER_POD,
                pv.into(),
                pvc.to_owned(),
                volume.to_str().unwrap().to_string(),
                stsname.to_owned(),
                passwd.to_owned(),
            );

            if delete.is_none() {
                apply_delete("create", &yaml).await?;
            } else {
                apply_delete("delete", &yaml).await?;
            }
        }
        Commands::Build {
            name,
            git,
            subpath,
            user_registry,
            passwd_registry,
            registry_api,
            image_name,
        } => {
            //创建一个configmap
            let mut registry_configmap = new_docker_registry(
                user_registry.as_str(),
                passwd_registry.as_str(),
                registry_api.to_string(),
                models::DOCKER_REGISTRY,
                "docker-reg".into(),
                "default".into(),
            );
            let build_yaml = new_kaniko_build(
                models::KANIKO_BUILD,
                name.to_string(),
                git.to_owned(),
                subpath.to_owned(),
                image_name.to_owned(),
                "docker-reg".to_string(),
                "default".to_string(),
            );
            // println!("{}",build_yaml);
            apply_delete("create", &registry_configmap).await?;
            apply_delete("create", &build_yaml).await?;
        }
    };
    Ok(())
}
