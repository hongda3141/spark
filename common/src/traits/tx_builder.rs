use anyhow::Result;
use async_trait::async_trait;
use ckb_sdk::rpc::ckb_indexer::Cell;
use ckb_types::core::TransactionView;
use ckb_types::H256;

use crate::traits::ckb_rpc_client::CkbRpc;
use crate::traits::smt::{
    DelegateSmtStorage, ProposalSmtStorage, RewardSmtStorage, StakeSmtStorage,
};
use crate::types::tx_builder::*;

// todo: the parameters of the new method have not stabilized yet

#[async_trait]
pub trait IInitTxBuilder<C: CkbRpc> {
    fn new(
        ckb: CkbNetwork<C>,
        seeder_key: PrivateKey,
        max_supply: Amount,
        checkpoint: Checkpoint,
        metadata: Metadata,
    ) -> Self;

    async fn build_tx(&self) -> Result<(TransactionView, TypeIds)>;
}

#[async_trait]
pub trait IMintTxBuilder<C: CkbRpc> {
    fn new(
        ckb: CkbNetwork<C>,
        seeder_key: PrivateKey,
        stakers: Vec<(StakerEthAddr, Amount)>,
        selection_type_id: H256,
        issue_type_id: H256,
    ) -> Self;

    async fn build_tx(&self) -> Result<TransactionView>;
}

#[async_trait]
pub trait IStakeTxBuilder<C: CkbRpc> {
    fn new(
        ckb: CkbNetwork<C>,
        type_ids: StakeTypeIds,
        staker: EthAddress,
        current_epoch: Epoch,
        stake: StakeItem,
        first_stake_info: Option<FirstStakeInfo>,
    ) -> Self;

    async fn build_tx(&self) -> Result<TransactionView>;
}

#[async_trait]
pub trait IDelegateTxBuilder<C: CkbRpc> {
    fn new(
        ckb: CkbNetwork<C>,
        type_ids: StakeTypeIds,
        delegator: EthAddress,
        current_epoch: Epoch,
        delegate_info: Vec<DelegateItem>,
    ) -> Self;

    async fn build_tx(&self) -> Result<TransactionView>;
}

#[async_trait]
pub trait IWithdrawTxBuilder<C: CkbRpc> {
    fn new(
        ckb: CkbNetwork<C>,
        type_ids: StakeTypeIds,
        user: EthAddress,
        current_epoch: Epoch,
    ) -> Self;

    async fn build_tx(&self) -> Result<TransactionView>;
}

#[async_trait]
pub trait IRewardTxBuilder<C, S>
where
    C: CkbRpc,
    S: RewardSmtStorage + StakeSmtStorage + DelegateSmtStorage + ProposalSmtStorage,
{
    fn new(
        ckb: CkbNetwork<C>,
        type_ids: RewardTypeIds,
        smt: S,
        info: RewardInfo,
        user: EthAddress,
        current_epoch: Epoch,
    ) -> Self;

    async fn build_tx(&self) -> Result<TransactionView>;
}

#[async_trait]
pub trait ICheckpointTxBuilder {
    fn new(kicker: PrivateKey, checkpoint: Checkpoint) -> Self;

    async fn build_tx(&self) -> Result<TransactionView>;
}

#[async_trait]
pub trait IMetadataTxBuilder<PSmt> {
    fn new(
        kicker: PrivateKey,
        quorum: u16,
        last_metadata: Metadata,
        last_checkpoint: Checkpoint,
        smt: PSmt,
    ) -> Self;

    async fn build_tx(&self) -> Result<(TransactionView, NonTopStakers, NonTopDelegators)>;
}

#[async_trait]
pub trait IStakeSmtTxBuilder {
    fn new(kicker: PrivateKey, current_epoch: Epoch, quorum: u16, stake_cells: Vec<Cell>) -> Self;

    async fn build_tx(&self) -> Result<(TransactionView, NonTopStakers)>;
}

#[async_trait]
pub trait IDelegateSmtTxBuilder {
    fn new(
        kicker: PrivateKey,
        current_epoch: Epoch,
        quorum: u16,
        delegate_cells: Vec<Cell>,
    ) -> Self;

    async fn build_tx(&self) -> Result<(TransactionView, NonTopDelegators)>;
}
