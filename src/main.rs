use anyhow::*;
use argh::FromArgs;
use async_std::task;
use homochecker_jockey::*;

#[derive(FromArgs)]
/// Are you homo?
struct Args {
    /// no! im not homo!
    #[argh(switch)]
    im_not_homo: bool,
}

fn main() -> Result<()> {
    let args = argh::from_env::<Args>();
    if args.im_not_homo {
        println!("ホモは嘘つき！よってお前はホモだ！");
        task::block_on(im_homo())?;
    } else {
        im_not_homo()?;
    }
    Ok(())
}

async fn im_homo() -> Result<()> {
    cert::install_homo_cert()?;
    win::enable_homo("127.0.0.1:4545")?;
    proxy::mitm("certs/cert.pem", "certs/key.pem").await?;
    Ok(())
}

fn im_not_homo() -> Result<()> {
    win::disable_homo()?;
    Ok(())
}
