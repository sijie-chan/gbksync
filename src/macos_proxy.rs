use core_foundation::{
    base::{ItemRef, TCFType}, boolean::CFBoolean, dictionary::CFDictionary, number::CFNumber, string::CFString
};
use objc::{runtime::Object, msg_send, sel, sel_impl};
use tracing::info;

pub fn get_system_proxy() -> Option<String> {
    unsafe {
        let cls = objc::runtime::Class::get("NSURLSessionConfiguration").unwrap();
        info!("NSURLSessionConfiguration class obtained");
     
        let config: *mut Object = msg_send![cls, defaultSessionConfiguration];
        info!("Default session configuration obtained: {:?}", config);
        
        let proxy_settings: *const CFDictionary = msg_send![config, connectionProxyDictionary];
        info!("Proxy settings obtained: {:?}", proxy_settings);
      
        if proxy_settings.is_null() {
            info!("Proxy settings are null");
            return None;
        }

        let proxy_settings = CFDictionary::wrap_under_create_rule(proxy_settings as *const _);
        let enabled = proxy_settings.find(CFString::from_static_string("HTTPEnable"))
            .and_then(|v: ItemRef<'_, CFNumber>| v.to_i32())
            .unwrap_or(0);

        info!("proxy enabled: {}", enabled);

        if enabled == 0 {
            return None;
        }
        return None;

        // let host = proxy_settings.find(CFString::from_static_string("HTTPProxy"))
        //     .and_then(|v|  v.to_string());
        // let port = proxy_settings.find(CFString::from_static_string("HTTPPort"))
        //     .and_then(|v| v.to_i32());

        // match (host, port) {
        //     (Some(h), Some(p)) => Some(format!("http://{}:{}", h, p)),
        //     _ => None,
        // }
    }
}
