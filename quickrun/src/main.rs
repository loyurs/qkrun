use apidocker;
use quickrun;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // apidocker::build_images_with_github("123", "qwdasd").await; //直接构建成了注册表信息
    //构建为tar,利用buildkit
}
