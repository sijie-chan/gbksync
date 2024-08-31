use std::process::Command;

pub fn get_http_proxy() -> Option<String> {
    // 获取所有网络服务名称
    let services = match Command::new("networksetup")
        .arg("-listallnetworkservices")
        .output()
    {
        Ok(output) => String::from_utf8_lossy(&output.stdout)
            .lines()
            .skip(1) // 跳过第一行（通常是提示信息）
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
        Err(_) => return None,
    };

    // 遍历所有服务，尝试获取 HTTP 代理设置
    for service in services {
        let output = Command::new("networksetup")
            .arg("-getwebproxy")
            .arg(&service)
            .output()
            .ok()?;

        if output.status.success() {
            let proxy_info = String::from_utf8_lossy(&output.stdout);

            // 解析代理信息
            let enabled = proxy_info.contains("Enabled: Yes");
            let server = proxy_info
                .lines()
                .find(|line| line.starts_with("Server:"))
                .and_then(|line| line.split_whitespace().nth(1));
            let port = proxy_info
                .lines()
                .find(|line| line.starts_with("Port:"))
                .and_then(|line| line.split_whitespace().nth(1));

            if enabled {
                if let (Some(server), Some(port)) = (server, port) {
                    return Some(format!("http://{}:{}", server, port));
                }
            }
        }
    }

    None
}

#[test]
fn test_get_http_proxy() {
    let proxy = get_http_proxy();
    println!("proxy: {}", proxy.unwrap());
}
