use serde::{Deserialize, Serialize, Serializer};
use subxt::{ClientBuilder, DefaultConfig, DefaultExtra};

#[subxt::subxt(runtime_metadata_path = "ternoa_metadata.scale")]
pub mod ternoa {}

pub const RPC_ENDPOINT: &str = "wss://127.0.0.1:443";

pub async fn get_block_data() -> Result<(), Box<dyn std::error::Error>> {
    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<ternoa::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>>();

    let mut iter = api.storage().nfts().data().await?;

    while let Some((key, account)) = iter.next().await? {
        println!("{}: {}", hex::encode(key), account.data.free);
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CliResponseFormat<T: Serialize> {
    pub status: bool,
    pub result: T,
}

impl<T> CliResponseFormat<T>
where
    T: Serialize,
{
    pub fn pretty_format(metadata: &CliResponseFormat<T>) -> Option<String> {
        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b" ");
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
        metadata.serialize(&mut ser).unwrap();
        String::from_utf8(ser.into_inner()).ok()
    }
}
