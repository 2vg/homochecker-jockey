use std::path::Path;

use anyhow::*;
use argus::{cert_util, proxy};

pub async fn mitm(cert: &str, key: &str) -> Result<()> {
    let cert = Path::new(cert);
    let key = Path::new(key);
    let pass = "114514";

    let container = cert_util::CAContainer::load_from_file(cert, key, pass.to_string())?;
    let mut p = proxy::Server::new("0.0.0.0:4545".parse().unwrap(), container);

    p.req_handler(|mut r| {
        Box::pin(async move {
            r.insert_header(
                "User-Agent",
                "Homozilla/5.0 (Checker/1.14.514; homOSeX 8.10)",
            );

            Ok(r)
        })
    });

    p.res_handler(|(_, mut res)| {
        Box::pin(async move {
            res.insert_header(
                "User-Agent",
                "Homozilla/5.0 (Checker/1.14.514; homOSeX 8.10)",
            );

            Ok(res)
        })
    });

    p.start().await
}
