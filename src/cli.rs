use clap::Parser;

use crate::usebuildkit::build_image;

#[derive(Parser)]
#[clap(name = "")]
#[clap(author = "Kevin K. <kbknapp@gmail.com>")]
#[clap(version = "1.0")]
#[clap(about = "Does awesome things", long_about = None)]
pub struct Cli {
    ///启动一个构建镜像构建，定义构建镜像的名字
    #[clap(long)]
    name: String,

    //是否需要扩展
    #[clap(long)]
    extensions: bool,
}


pub fn cli_run() {
    let cmd_args = Cli::parse();
    println!("{:?}",cmd_args.name);
    build_image(&cmd_args.name);
}