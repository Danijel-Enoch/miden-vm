use super::{fmt, hasher, Box, CodeBlock, Digest, Felt, Operation};

// JOIN BLOCKS
// ================================================================================================
/// A code block used to combine two other code blocks.
///
/// When the VM executes a Join block, it executes joined blocks in sequence one after the other.
///
/// Hash of a Join block is computed by hashing a concatenation of the hashes of joined blocks.
#[derive(Clone, Debug)]
pub struct Join {
    body: Box<[CodeBlock; 2]>,
    hash: Digest,
}

impl Join {
    // CONSTANTS
    // --------------------------------------------------------------------------------------------
    /// The domain of the join block (used for control block hashing).
    pub const DOMAIN: Felt = Felt::new(Operation::Join.op_code() as u64);

    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    /// Returns a new [Join] block instantiated with the specified code blocks.
    pub fn new(body: [CodeBlock; 2]) -> Self {
        let hash = hasher::merge_in_domain(&[body[0].hash(), body[1].hash()], Self::DOMAIN);
        Self {
            body: Box::new(body),
            hash,
        }
    }

    // PUBLIC ACCESSORS
    // --------------------------------------------------------------------------------------------

    /// Returns a hash of this code block.
    pub fn hash(&self) -> Digest {
        self.hash
    }

    /// Returns a reference to the code block which is to be executed first when this join block
    /// is executed.
    pub fn first(&self) -> &CodeBlock {
        &self.body[0]
    }

    /// Returns a reference to the code block which is to be executed second when this join block
    /// is executed.
    pub fn second(&self) -> &CodeBlock {
        &self.body[1]
    }

    /// Returns the domain of the join block
    pub const fn domain() -> Felt {
        Operation::Join.domain()
    }
}

impl fmt::Display for Join {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "join {} {} end", self.body[0], self.body[1])
    }
}
