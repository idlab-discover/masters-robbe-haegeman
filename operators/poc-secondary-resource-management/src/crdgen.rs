use kube::CustomResourceExt;
use std::fs::File;

use controller::crd;

fn main() {
    let path = std::path::Path::new("crd/database.yaml");
    let prefix = path.parent().unwrap();
    println!("{:?}", path);
    std::fs::create_dir_all(prefix).unwrap();
    let file = File::create(path).unwrap();
    serde_yaml::to_writer(file, &crd::Database::crd()).unwrap();
}
