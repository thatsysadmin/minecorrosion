use serde_json::Value;

pub struct PullDownloadConfiguration {
    is_fabric: bool,
    download_path: String,
    hash: HashType,
    file_size: u64,
    url: String,
    path: Option<String>,
}

impl PullDownloadConfiguration {
    pub fn builder(download_configuration_json: Value) -> Self {
        match download_configuration_json.get("artifact") {
            Some(artifact_wrapped) => { // Vanilla or Neoforge configuration.
                panic!()
            }
            None => { // Fabric configuration - Dunno how this works at all.
                panic!()
            }
        }

        return PullDownloadConfiguration {
            is_fabric: false,

        }
    }

    pub fn classpath_argument(&self) -> String {
        panic!("Unimplemented.")
    }

    pub fn download(&self) -> Result<(), ()> {
        if self.is_fabric { // Need to dynamically generate from maven repo

        }
        else {

        }
        panic!("Unimplemented.")
    }
}
 
enum HashType {
    SHA1(String),
    SHA256(String),
    SHA512(String),
}