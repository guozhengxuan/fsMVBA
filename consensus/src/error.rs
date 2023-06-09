use crate::{messages::Proof, config::{EpochNumber, ViewNumber}, ConsensusMessage};
use crypto::{CryptoError, Digest, PublicKey};
use store::StoreError;
use thiserror::Error;

#[macro_export]
macro_rules! bail {
    ($e:expr) => {
        return Err($e)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! ensure {
    ($cond:expr, $e:expr) => {
        if !($cond) {
            bail!($e);
        }
    };
}

pub type ConsensusResult<T> = Result<T, ConsensusError>;

#[derive(Error, Debug)]
pub enum ConsensusError {
    #[error("Network error: {0}")]
    NetworkError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] Box<bincode::ErrorKind>),

    #[error("Invalid digest to retrieve block.")]
    DigestError,

    #[error("Store error: {0}")]
    StoreError(#[from] StoreError),

    #[error("Node {0} is not in the committee")]
    NotInCommittee(PublicKey),

    #[error("Consensus message with halted epoch: {0}, currently lowest epoch not halted: {1}")]
    MessageWithHaltedEpoch(EpochNumber, EpochNumber),

    #[error("Invalid vote proof")]
    InvalidVoteProof(Proof),

    #[error("Invalid signature")]
    InvalidSignature(#[from] CryptoError),

    #[error("Invalid signature share from {0}")]
    InvalidSignatureShare(PublicKey),

    #[error("Invalid threshold signature from {0}")]
    InvalidThresholdSignature(PublicKey),

    #[error("Random coin with wrong leader")]
    RandomCoinWithWrongLeader,

    #[error("Random coin with wrong shares")]
    RandomCoinWithWrongShares,

    #[error("Received repeated vote {1} from {0}")]
    AuthorityReuseinQC(PublicKey, ConsensusMessage),

    #[error("Received more than one timeout from {0}")]
    AuthorityReuseinTC(PublicKey),

    #[error("Received more than one random share from {0}")]
    AuthorityReuseinCoin(PublicKey),

    #[error("Received vote from unknown authority {0}")]
    UnknownAuthority(PublicKey),

    #[error("Received QC without a quorum")]
    QCRequiresQuorum,

    #[error("Received TC without a quorum")]
    TCRequiresQuorum,

    #[error("Received RandomCoin without a quorum")]
    RandomCoinRequiresQuorum,

    #[error("Malformed block {0}")]
    MalformedBlock(Digest),

    #[error("Echo of block {digest} of leader {leader} received by {author} at epoch {epoch}, view {view}")]
    WrongLeader {
        digest: Digest,
        leader: PublicKey,
        author: PublicKey,
        epoch: EpochNumber,
        view: ViewNumber,
    },

    #[error("Invalid payload")]
    InvalidPayload,
}
