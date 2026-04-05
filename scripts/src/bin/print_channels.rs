use cw_orch::{
    contract::interface_traits::ContractInstance,
    daemon::{
        networks::{ARCHWAY_1, JUNO_1, NEUTRON_1, OSMOSIS_1, PHOENIX_1},
        queriers::Ibc,
        Daemon,
    },
    prelude::*,
    tokio::runtime::{Handle, Runtime},
};
use cw_orch_polytone::Polytone;
use polytone_note::msg::QueryMsgFns as _;
use scripts::{helpers::get_deployment_id, KAIYO_1, STARGAZE_1};

fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    // pretty_env_logger::init();
    let src_chain = KAIYO_1;

    let dst_chains = [
        OSMOSIS_1, JUNO_1, ARCHWAY_1, NEUTRON_1, PHOENIX_1, STARGAZE_1,
    ];

    let rt = Runtime::new()?;

    println!("{{");

    for chain in dst_chains {
        query_one_pair(rt.handle(), src_chain.clone(), chain.clone())?;
        query_one_pair(rt.handle(), chain.clone(), src_chain.clone())?;
    }
    println!("}}");

    Ok(())
}

fn query_one_pair(
    handle: &Handle,
    src_chain: ChainInfo,
    dst_chain: ChainInfo,
) -> anyhow::Result<()> {
    // We query the src channel
    let deployment_id = get_deployment_id(&src_chain, &dst_chain);

    let src_daemon = Daemon::builder(src_chain)
        .deployment_id(deployment_id.clone())
        .handle(handle)
        .build()?;

    let dst_daemon = Daemon::builder(dst_chain)
        .deployment_id(deployment_id.clone())
        .state(src_daemon.state())
        .handle(handle)
        .build()?;

    let src_polytone = Polytone::load_from(src_daemon.clone())?;

    let src_channel = src_polytone.note.active_channel()?.unwrap();

    let src_port = format!("wasm.{}", src_polytone.note.addr_str()?);

    let channel_description =
        handle.block_on(Ibc::new(&src_daemon)._channel(src_port.clone(), src_channel.clone()))?;

    let counterparty = channel_description.counterparty.unwrap();

    // We get the counterparty connection as well
    let counter_party_connection = handle.block_on(Ibc::new(&dst_daemon)._channel(
        counterparty.port_id.clone(),
        counterparty.channel_id.clone(),
    ))?;

    println!(
        "\"{deployment_id}\": {{
            \"src\":{{
                \"connection\": \"{}\",
                \"port_id\": \"{}\",
                \"channel_id\": \"{}\"
            }},
            \"dst\":{{
                \"connection\": \"{}\",
                \"port_id\": \"{}\",
                \"channel_id\": \"{}\"
            }}
        }},",
        channel_description.connection_hops[0],
        src_port,
        src_channel,
        counter_party_connection.connection_hops[0],
        counterparty.port_id.clone(),
        counterparty.channel_id.clone(),
    );
    Ok(())
}
