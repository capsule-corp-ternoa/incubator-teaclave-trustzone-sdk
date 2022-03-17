use optee_teec::{Context, ErrorKind, Operation, ParamNone, ParamTmpRef, Session, Uuid};
use proto::{Command, UUID};
use std::ffi::CString;
mod ternoa_types;
mod rpc;
use subxt::{
    ClientBuilder,
    DefaultConfig,
    DefaultExtra,
};
use serde::{Deserialize, Serialize, Serializer};

#[subxt::subxt(runtime_metadata_path = "ternoa_metadata.scale")]
pub mod ternoa {}

const TEST_OBJECT_SIZE: usize = 7000;

fn read_secure_object(
    session: &mut Session,
    obj_id: &[u8],
    obj_data: &mut [u8],
) -> optee_teec::Result<()> {
    let p0 = ParamTmpRef::new_input(obj_id);
    let p1 = ParamTmpRef::new_output(obj_data);
    let mut operation = Operation::new(0, p0, p1, ParamNone, ParamNone);

    session.invoke_command(Command::Read as u32, &mut operation)?;

    println!("- Read back the object");
    Ok(())
}

fn write_secure_object(
    session: &mut Session,
    obj_id: &[u8],
    obj_data: &[u8],
) -> optee_teec::Result<()> {
    let p0 = ParamTmpRef::new_input(obj_id);
    let p1 = ParamTmpRef::new_input(obj_data);
    let mut operation = Operation::new(0, p0, p1, ParamNone, ParamNone);

    session.invoke_command(Command::Write as u32, &mut operation)?;

    println!("- Create and load object in the TA secure storage");
    Ok(())
}

async fn demo() -> optee_teec::Result<()> {
    let mut ctx = Context::new()?;
    let uuid = Uuid::parse_str(UUID).unwrap();
    let mut session = ctx.open_session(uuid)?;

    let api = ClientBuilder::new()
    .build()
    .await?
    .to_runtime_api::<ternoa::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>>();

    
    // Subscribe to any events that occur:
    let mut event_sub = api.events().subscribe().await?;

    while let Some(events) = event_sub.next().await {

        let events = events?;
        let block_hash = events.block_hash();

        for event in events.iter_raw() {
            let event = event?;
            let is_balance_transfer = event
                .as_event::<ternoa::nfts::events::Created>()?
                .is_some();
            let pallet = event.pallet;
            let variant = event.variant;
            println!("NFT Creation detected!");

            // TODO : Find better way to encode nft data to enclave
            let obj1_id = CString::new(variant.nft_id).unwrap().into_bytes_with_nul();
            let obj1_data = rpc::get_block_data().into();
            let mut read_data = [0x00u8; TEST_OBJECT_SIZE];

            println!("\nTest on object \"object#1\"");
            write_secure_object(&mut session, obj1_id.as_slice(), &obj1_data)?;
            read_secure_object(&mut session, obj1_id.as_slice(), &mut read_data)?;

            if obj1_data.iter().zip(read_data.iter()).all(|(a, b)| a == b) {
                println!("- Content read-out correctly");
                } else {
                println!("- Unexpected content found in secure storage");
                }
            }
        }
    
    Ok(())
}

