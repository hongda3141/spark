#[cfg(test)]
mod tests {
    use ckb_types::h256;

    use common::traits::tx_builder::IInitTxBuilder;
    use common::types::tx_builder::{Checkpoint, CkbNetwork, Metadata, NetworkType};
    use rpc_client::ckb_client::ckb_rpc_client::CkbRpcClient;

    use crate::ckb::init::InitTxBuilder;
    use crate::ckb::utils::tx::send_tx;

    // #[tokio::test]
    async fn _send_init_tx() {
        let test_seeder_key =
            h256!("0x13b08bb054d5dd04013156dced8ba2ce4d8cc5973e10d905a228ea1abc267e61");
        let ckb_client = CkbRpcClient::new("https://testnet.ckb.dev");

        let (tx, type_id_args) = InitTxBuilder::new(
            CkbNetwork {
                network_type: NetworkType::Testnet,
                client:       ckb_client.clone(),
            },
            test_seeder_key,
            10000,
            Checkpoint {
                epoch: 1,
                ..Default::default()
            },
            Metadata::default(),
        )
        .build_tx()
        .await
        .unwrap();

        // println!("tx: {}", tx);

        match send_tx(&ckb_client, &tx.data().into()).await {
            Ok(tx_hash) => {
                println!("tx hash: 0x{}", tx_hash);
                println!("selection type id: 0x{}", type_id_args.selection_type_id);
                println!("issue type id: 0x{}", type_id_args.issue_type_id);
                println!("checkpoint type id: 0x{}", type_id_args.checkpoint_type_id);
                println!("metadata type id: 0x{}", type_id_args.metadata_type_id);
                println!("xudt owner: 0x{}", type_id_args.xudt_owner);
            }
            Err(e) => println!("{}", e),
        }
    }
}
