use git2::{Commit, Config, Cred, CredentialHelper, Credentials, Oid, ProxyOptions, Repository};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::init("./")?;
    let config = Config::new()?;

    let mut proxy_opt = ProxyOptions::new();
    proxy_opt.url("http://127.0.0.1:6512");

    let mut credential_helper = CredentialHelper::new("https://github.com/qinyuhang/novel.git");

    if let Some(res) = credential_helper.execute() {
        println!("res: {:?}", res);
    }

    Ok(())
}
