//启动一个vscode-web-server
use serde::Serialize;
use tinytemplate::TinyTemplate;
use tracing::info;

use crate::k8sapi::apply_delete;

///配置模板文件参数
#[derive(Serialize, Default)]
pub struct VscodeServerPod {
    pub pv_name: String,
    pub host_path: String,
    pub pvc_name: String,
    pub statefulset_name: String,
    pub vscode_password: String,
}
pub static VSCODE_SERVER_POD: &str = r#"kind: PersistentVolume
apiVersion: v1
metadata:
  name: {pv_name}
  labels:
    type: local
spec:
  storageClassName: {pv_name}
  capacity:
    storage: 20Gi
  accessModes:
    - ReadWriteOnce
  persistentVolumeReclaimPolicy: Retain
  hostPath:
    path: "{host_path}"
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: {pvc_name}
spec:
  storageClassName: {pv_name}
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 20Gi
---
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: {statefulset_name}
spec:
  serviceName: {statefulset_name}
  replicas: 1
  selector:
    matchLabels:
      app: rust
  template:
    metadata:
      labels:
        app: rust
    spec:
      containers:
      - name: {statefulset_name}
        image: registry.cn-hangzhou.aliyuncs.com/clouddevs/code-server-vscode:latest
        imagePullPolicy: IfNotPresent
        env:
        - name: PASSWORD
          value: {vscode_password}
        - name: SUDO_PASSWORD
          value: {vscode_password}
        - name: DEFAULT_WORKSPACE
          value: /config/workspace
        - name: PUID
          value: "0"
        - name: PGID
          value: "0"
        - name: HOME
          value: /config
        #挂载点注意，连接vscode-client，默认配置文件在 /root/.vscode-server
        ports:
        - containerPort: 8443
          name: code-server
        - containerPort: 8022
          name: ssh-port
        volumeMounts:
        - name: rust-config
          mountPath: /config
      dnsPolicy: ClusterFirst
      volumes:
      volumes:
      - name: rust-config
        persistentVolumeClaim:
          claimName: {pvc_name}
---
apiVersion: v1
kind: Service
metadata:
  name: service4rust
spec:
  type: NodePort
  selector:
    app: rust
  ports:
    - port: 8022
      targetPort: 8022
      name: ssh-port
    - port: 8443
      name: code-server
      targetPort: 8443
"#;
///为此模板进行实例化
pub fn new_vscode_server_pod(
    template: &str,
    pv_name: String,
    pvc_name: String,
    host_path: String,
    statefulset_name: String,
    vscode_password: String,
) -> String {
    let mut tt = TinyTemplate::new();
    tt.add_template("vscode_server_pod", template).unwrap();
    let kaka = VscodeServerPod {
        pv_name,
        host_path,
        pvc_name,
        statefulset_name,
        vscode_password,
    };
    tt.render("vscode_server_pod", &kaka).unwrap()
}

#[test]
fn test_new_vscode_server_pod() {}

///kaniko构建模板
//context_git_url 为   String(- '--context-sub-path=).push_str("git_url")
//sub_path
#[derive(Serialize, Default)]
struct KanikoBuild {
    kaniko_build_name: String,
    context_git_url: String,
    sub_path: String, //r#"- '--context-sub-path=dockerfiles/pp/'"#   sub_path的格式
    image_name: String,
    cm_name: String,
    ns: String,
}

pub static KANIKO_BUILD: &str = r#"apiVersion: v1
kind: Pod
metadata:
  name: {kaniko_build_name}
  namespace: {ns}
spec:
  containers:
    - name: {kaniko_build_name}
      image: "registry.cn-hangzhou.aliyuncs.com/clouddevs/kanico:latest"
      imagePullPolicy: IfNotPresent
      #   stdin: false
      #stdinOnce: true
      args:
        - '--dockerfile=Dockerfile'
        - '--context={context_git_url}'
        - '--context-sub-path={sub_path}'
        - '--destination={image_name}'
      volumeMounts:
        - name: docker-config
          mountPath: /kaniko/.docker/
  restartPolicy: Never
  volumes:
    - name: docker-config
      configMap:
        name: {cm_name}
"#;

pub fn new_kaniko_build(
    template: &str,
    kaniko_build_name: String,
    context_git_url: String,
    sub_path: String,
    image_name: String,
    cm_name: String,
    ns: String,
) -> String {
    let kaniko_build = KanikoBuild {
        kaniko_build_name,
        context_git_url,
        sub_path,
        image_name,
        cm_name,
        ns,
    };
    let mut tt = TinyTemplate::new();
    tt.add_template("kaniko_build", template).unwrap();
    tt.render("kaniko_build", &kaniko_build).unwrap()
}

#[test]
fn test_new_kaniko_build() -> Result<(), anyhow::Error> {
    // let pp = new_kaniko_build(KANIKO_BUILD, "mykani".into(), "git://github.com/loyurs/qkrun.git#refs/heads/master".into(), "build_images/dockerfiles/code_server_with_ssh/".into(),"ccr.ccs.tencentyun.com/loyu/litong1:latest".into(),"docker-config".into(),"default".into());
    // println!("{}", pp);
    let pp = new_kaniko_build(
        KANIKO_BUILD,
        "mykani".into(),
        "git://github.com/loyurs/posmtp.git#".into(),
        "".into(),
        "ccr.ccs.tencentyun.com/loyu/litong1:latest".into(),
        "docker-config".into(),
        "default".into(),
    );

    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(apply_delete("create", &pp))?;
    Ok(())
}

#[derive(Serialize)]
pub struct DockerRegistry {
    based64: String,
    api_url: String,
    cm_name: String,
    ns: String,
}
///docker 私有仓库凭证
pub static DOCKER_REGISTRY: &str = r#"apiVersion: v1
data:
  config.json: |
    \{
        "auths": \{
            "{api_url}": \{
                "auth": "{based64}"
            }
        }
    }
kind: ConfigMap
metadata:
  name: {cm_name}
  namespace: {ns}"#;

// 如果模板包含左大括号 （），则必须使用前导字符对大括号进行转义。例如：{\

//   h2 \{
//       font-size: {fontsize};
//   }

fn based64go<'a>(user: &'a str, passwd: &'a str) -> String {
    let c = DOCKER_REGISTRY;
    let based64 = base64::encode(format!("{}:{}", user, passwd));
    based64
}
///生成添加了用户名密码的configmap yaml配置文件
pub fn new_docker_registry(
    user: &str,
    password: &str,
    api_url: String,
    template: &str,
    cm_name: String,
    ns: String,
) -> String {
    info!(
        "namespace:{}, configmap_name: {} should be the same with mounted pod",
        ns, cm_name
    );
    let based64 = based64go(user, password);
    let docker_registry = DockerRegistry {
        based64,
        api_url,
        ns,
        cm_name,
    };
    let mut tt = TinyTemplate::new();
    tt.add_template("docker_registry", template).unwrap();
    tt.render("docker_registry", &docker_registry).unwrap()
}

#[test]
fn test_new_docker_registry() {
    let pc = new_docker_registry(
        "100016367772",
        "***",
        "ccr.ccs.tencentyun.com".into(),
        DOCKER_REGISTRY,
        "docker-config".into(),
        "default".into(),
    );
    // println!("{}",pc);
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(apply_delete("create", &pc))
        .unwrap();
}

//根据信息创建secret
//https://github.com/GoogleContainerTools/kaniko#kubernetes-secret
#[derive(Serialize)]
struct KanikoSecret {
    json_based64: String,
    secret_name: String,
}

static KANIKO_SECRET: &str = r#"apiVersion: v1
data:
  config.json: {json_based64}
kind: Secret
metadata:
  name: {secret_name}
  namespace: default
type: Opaque"#;

pub fn new_secret(
    template: &str,
    json_based64: String,
    secret_name: String,
) -> Result<String, anyhow::Error> {
    let mut tt = TinyTemplate::new();
    tt.add_template("secret", template)?;
    let kaniko_secret = KanikoSecret {
        json_based64,
        secret_name,
    };
    let secret = tt.render("secret", &kaniko_secret)?;
    Ok(secret)
}

#[test]
fn test_new_secret() {
    // let sc = new_secret(KANIKO_SECRET, "213", "kate".into()).unwrap();
}
