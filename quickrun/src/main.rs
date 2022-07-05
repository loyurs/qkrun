use apidocker;
use quickrun;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    quickrun::usebuildkit::dockerd_buildkit_build(
        "asd",
        "https://github.com/loyurs/qkrun.git#master:dockerfiles/code_server_with_ssh/",
        "/home/:/home",
    );

    //构建为tar,利用buildkit
}
