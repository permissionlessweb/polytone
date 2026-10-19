use cw_orch::{prelude::networks::*, prelude::*, tokio::runtime::Runtime};
use cw_orch_polytone::Polytone;

fn main() {
    // Terp lab/testnet first, then mainnet. Upload-only: note, voice, proxy
    // (Polytone::store_on). Instantiation is out of scope for this pass.
    for chain in [TERP_TESTNET, TERP_MAINNET] {
        eprintln!("uploading polytone to {}", chain.chain_id);
        if let Err(e) = upload_contracts(chain) {
            eprintln!("upload failed on {}: {e:?}", chain.chain_id);
            std::process::exit(1);
        }
    }
}

fn upload_contracts(chain: ChainInfo) -> anyhow::Result<()> {
    let _ = dotenv::dotenv();
    // init once is fine; ignore already-init
    let _ = pretty_env_logger::try_init();
    let rt = Runtime::new()?;
    let daemon = DaemonBuilder::new(chain).handle(rt.handle()).build()?;

    let _polytone = Polytone::store_on(daemon)?;

    Ok(())
}
