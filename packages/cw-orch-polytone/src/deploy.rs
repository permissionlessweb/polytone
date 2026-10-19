use std::path::PathBuf;

use crate::utils::read_json;
use crate::{PolytoneNote, PolytoneProxy, PolytoneVoice};
use cw_orch::core::serde_json::Value;
use cw_orch::daemon::DeployedChains;
use cw_orch::prelude::*;

use crate::Polytone;

pub const POLYTONE_NOTE: &str = "polytone:note";
pub const POLYTONE_VOICE: &str = "polytone:voice";
pub const POLYTONE_PROXY: &str = "polytone:proxy";

pub const MAX_BLOCK_GAS: u64 = 100_000_000;

impl<Chain: CwEnv> Deploy<Chain> for Polytone<Chain> {
    type Error = CwOrchError;

    type DeployData = Empty;

    fn store_on(chain: Chain) -> Result<Self, <Self as Deploy<Chain>>::Error> {
        let polytone = Polytone::new(chain);

        polytone.note.upload()?;
        polytone.voice.upload()?;
        polytone.proxy.upload()?;

        Ok(polytone)
    }

    fn deploy_on(chain: Chain, _data: Self::DeployData) -> Result<Self, CwOrchError> {
        // Deployment of Polytone is simply uploading the contracts
        let deployment = Self::store_on(chain.clone())?;

        Ok(deployment)
    }

    fn get_contracts_mut(
        &mut self,
    ) -> Vec<Box<&mut dyn cw_orch::prelude::ContractInstance<Chain>>> {
        vec![
            Box::new(&mut self.note),
            Box::new(&mut self.voice),
            Box::new(&mut self.proxy),
        ]
    }

    fn load_from(chain: Chain) -> Result<Self, Self::Error> {
        // This only loads the code-ids, because this structure only holds Polytone Code Ids
        let mut polytone = Self::new(chain.clone());
        // We register all the code_id default state
        if chain.can_load_state_from_state_file() {
            polytone.set_contracts_state(None);
        }

        Ok(polytone)
    }
}

impl<Chain: CwEnv> DeployedChains<Chain> for Polytone<Chain> {
    /// This allows loading only the code_ids from the state, because addresses are not relevant for this Structure
    fn set_contracts_state(&mut self, custom_state: Option<Value>) {
        let state;

        let state_file = Self::deployed_state_file_path();
        if let Some(custom_state) = custom_state {
            state = custom_state;
        } else if let Some(state_file) = state_file {
            if let Ok(module_state_json) = read_json(&state_file) {
                state = module_state_json;
            } else {
                return;
            }
        } else {
            return;
        }

        let all_contracts = self.get_contracts_mut();

        for contract in all_contracts {
            // We set the code_id and/or address of the contract in question if they are not present already
            let env_info = contract.environment().env_info();
            // We load the file
            // We try to get the code_id for the contract
            if contract.code_id().is_err() {
                let code_id = state
                    .get(env_info.chain_id.to_string())
                    .unwrap_or(&Value::Null)
                    .get("code_ids")
                    .unwrap_or(&Value::Null)
                    .get(contract.id());

                if let Some(code_id) = code_id {
                    if code_id.is_u64() {
                        contract.set_default_code_id(code_id.as_u64().unwrap())
                    }
                }
            }
        }
    }

    fn deployed_state_file_path() -> Option<String> {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        Some(
            PathBuf::from(crate_path)
                .join("cw-orch-state.json")
                .display()
                .to_string(),
        )
    }
}

impl<Chain: CwEnv> Polytone<Chain> {
    pub fn new(chain: Chain) -> Self {
        let note = PolytoneNote::new(POLYTONE_NOTE, chain.clone());
        let voice = PolytoneVoice::new(POLYTONE_VOICE, chain.clone());
        let proxy = PolytoneProxy::new(POLYTONE_PROXY, chain.clone());

        Polytone { note, voice, proxy }
    }

    pub fn store_if_needed(chain: Chain) -> Result<Self, <Self as Deploy<Chain>>::Error> {
        let polytone = Polytone::load_from(chain)?;

        polytone.note.upload_if_needed()?;
        polytone.voice.upload_if_needed()?;
        polytone.proxy.upload_if_needed()?;

        Ok(polytone)
    }

    pub fn instantiate_note(
        &self,
        admin: Option<String>,
    ) -> Result<Chain::Response, CwOrchError> {
        self.note.instantiate(
            &polytone_note::msg::InstantiateMsg {
                pair: None,
                block_max_gas: MAX_BLOCK_GAS.into(),
            },
            admin.map(Addr::unchecked).as_ref(),
            &[],
        )
    }

    pub fn instantiate_voice(
        &self,
        admin: Option<String>,
    ) -> Result<Chain::Response, CwOrchError> {
        self.voice.instantiate(
            &polytone_voice::msg::InstantiateMsg {
                proxy_code_id: self.proxy.code_id()?.into(),
                block_max_gas: MAX_BLOCK_GAS.into(),
            },
            admin.map(Addr::unchecked).as_ref(),
            &[],
        )
    }
}
