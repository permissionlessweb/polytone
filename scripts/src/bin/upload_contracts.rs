use cw_orch::{prelude::networks::*, prelude::*, tokio::runtime::Runtime};
use cw_orch_polytone::Polytone;

fn main() {
    let chain = PHOENIX_1;
    upload_contracts(chain).unwrap();
}

fn upload_contracts(chain: ChainInfo) -> anyhow::Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::init();
    let rt = Runtime::new()?;
    let daemon = DaemonBuilder::new(chain).handle(rt.handle()).build()?;

    let _polytone = Polytone::store_on(daemon)?;

    Ok(())
}
