#[warn(unused)]

use clap::Parser;


use crate::usebuildkit::{build_rust_dev_image_with_extensions, giturl_branch_and_folder};

#[derive(Parser)]
#[clap(name = "")]
#[clap(author = "loyu loyurs@163.com")]
#[clap(version = "0.0.1")]
#[clap(about = "make a docker vscode-server", long_about = None)]
pub struct Cli {
    // ///启动一个构建镜像构建，定义构建镜像的名字
    // #[clap(short,long)]
    // name: String,

    // ///是否需要扩展
    // #[clap(short,long)] //default_value="No"
    // extensions: Option<String>,

    #[clap(short, long)]
    app: String,

}


pub fn cli_run() {
    let cmd_args = Cli::parse();
    // build_rust_dev_image_with_extensions("hahademo");

}