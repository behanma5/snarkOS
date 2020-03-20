use crate::dpc::plain_dpc::{DPCComponents, PlainDPCComponents};

use snarkos_models::algorithms::SNARK;

#[derive(Derivative)]
#[derivative(Clone(bound = "C: DPCComponents"))]
pub struct CommAndCRHPublicParameters<C: DPCComponents> {
    pub addr_comm_pp: C::AddressCommitment,
    pub rec_comm_pp: C::RecordCommitment,
    pub pred_vk_comm_pp: C::PredicateVerificationKeyCommitment,
    pub local_data_comm_pp: C::LocalDataCommitment,

    pub sn_nonce_crh_pp: C::SerialNumberNonce,
    pub pred_vk_crh_pp: C::PredicateVerificationKeyHash,
}

#[derive(Derivative)]
#[derivative(Clone(bound = "C: PlainDPCComponents"))]
pub struct PredNIZKParameters<C: PlainDPCComponents> {
    pub pk: <C::PredicateNIZK as SNARK>::ProvingParameters,
    pub vk: <C::PredicateNIZK as SNARK>::VerificationParameters,
    pub proof: <C::PredicateNIZK as SNARK>::Proof,
}

pub struct PublicParameters<C: PlainDPCComponents> {
    pub comm_and_crh_pp: CommAndCRHPublicParameters<C>,
    pub pred_nizk_pp: PredNIZKParameters<C>,
    pub proof_check_nizk_pp: (
        <C::ProofCheckNIZK as SNARK>::ProvingParameters,
        <C::ProofCheckNIZK as SNARK>::PreparedVerificationParameters,
    ),
    pub core_nizk_pp: (
        <C::MainNIZK as SNARK>::ProvingParameters,
        <C::MainNIZK as SNARK>::PreparedVerificationParameters,
    ),
}

impl<C: PlainDPCComponents> PublicParameters<C> {
    pub fn core_check_nizk_pp(
        &self,
    ) -> &(
        <C::MainNIZK as SNARK>::ProvingParameters,
        <C::MainNIZK as SNARK>::PreparedVerificationParameters,
    ) {
        &self.core_nizk_pp
    }

    pub fn proof_check_nizk_pp(
        &self,
    ) -> &(
        <C::ProofCheckNIZK as SNARK>::ProvingParameters,
        <C::ProofCheckNIZK as SNARK>::PreparedVerificationParameters,
    ) {
        &self.proof_check_nizk_pp
    }

    pub fn pred_nizk_pp(&self) -> &PredNIZKParameters<C> {
        &self.pred_nizk_pp
    }

    pub fn sn_nonce_crh_pp(&self) -> &C::SerialNumberNonce {
        &self.comm_and_crh_pp.sn_nonce_crh_pp
    }

    pub fn pred_vk_crh_pp(&self) -> &C::PredicateVerificationKeyHash {
        &self.comm_and_crh_pp.pred_vk_crh_pp
    }

    pub fn local_data_comm_pp(&self) -> &C::LocalDataCommitment {
        &self.comm_and_crh_pp.local_data_comm_pp
    }

    pub fn addr_comm_pp(&self) -> &C::AddressCommitment {
        &self.comm_and_crh_pp.addr_comm_pp
    }

    pub fn rec_comm_pp(&self) -> &C::RecordCommitment {
        &self.comm_and_crh_pp.rec_comm_pp
    }

    pub fn pred_vk_comm_pp(&self) -> &C::PredicateVerificationKeyCommitment {
        &self.comm_and_crh_pp.pred_vk_comm_pp
    }
}
