use anyhow::*;
use hyper::{
    header::USER_AGENT, service::Service,
    http::{HeaderValue, Request}
};
use hyper::Body;
use third_wheel::*;

pub async fn mitm(cert: impl Into<String>, key: impl Into<String>) -> Result<()> {
    let ca = CertificateAuthority::load_from_pem_files_with_passphrase_on_key(
        cert.into(),
        key.into(),
        "114514",
    )?;
    let mitm =
        mitm_layer(|mut req: Request<Body>, mut third_wheel: ThirdWheel|{
            req.headers_mut().insert(USER_AGENT, "Homozilla/5.0 (Checker/1.14.514; homOSeX 8.10)".parse().unwrap());
            third_wheel.call(req)
        });
    let mitm_proxy = MitmProxy::builder(mitm, ca).build();
    let (_, mitm_proxy_fut) = mitm_proxy.bind(format!("127.0.0.1:4545").parse()?);
    mitm_proxy_fut.await?;

    Ok(())
}
