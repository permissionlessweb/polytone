use cw_orch_interchain::mock::MockInterchainEnv;
use cw_orch_polytone::PolytoneConnection;

#[test]
fn mock_uses_state() -> anyhow::Result<()> {
    let interchain = MockInterchainEnv::new(vec![("osmosis-1", "osmo"), ("juno-1", "juno")]);

    PolytoneConnection::deploy_between_if_needed(&interchain, "osmosis-1", "juno-1")?;

    Ok(())
}
