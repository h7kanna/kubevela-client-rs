use kubevela::apis::*;
use kubevela::models::*;

fn main() {
  let mut kubevela_config = configuration::Configuration::default();
  kubevela_config.bearer_access_token = Some("my token".to_string());
  kubevela_config.base_path = "http://192.168.86.123:8080/".to_string();
}
