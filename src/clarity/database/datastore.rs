use super::{ClarityBackingStore, ClarityDatabase, HeadersDB};
use crate::clarity::analysis::AnalysisDatabase;
use crate::clarity::errors::{
    CheckErrors, IncomparableError, InterpreterError, InterpreterResult as Result, RuntimeErrorType,
};
use crate::clarity::types::QualifiedContractIdentifier;
use crate::clarity::util::hash::Sha512Trunc256Sum;
use crate::clarity::StacksBlockId;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Datastore {
    store: HashMap<String, String>,
    metadata: HashMap<(String, String), String>,
    chain_tip: StacksBlockId,
    chain_height: u32,
}

impl Datastore {
    pub fn new() -> Datastore {
        Datastore {
            store: HashMap::new(),
            metadata: HashMap::new(),
            chain_tip: StacksBlockId([0u8; 32]),
            chain_height: 0,
        }
    }

    pub fn advance_chain_tip(&mut self, count: u32) -> u32 {
        self.chain_height = self.chain_height + count;
        let chain_height_bytes = self.chain_height.to_be_bytes();
        let mut bytes = [0u8; 32];
        bytes[0] = chain_height_bytes[0];
        bytes[1] = chain_height_bytes[1];
        bytes[2] = chain_height_bytes[2];
        bytes[3] = chain_height_bytes[3];
        self.chain_tip = StacksBlockId(bytes);
        self.chain_height.clone()
    }
}

impl ClarityBackingStore for Datastore {
    fn put_all(&mut self, items: Vec<(String, String)>) {
        for (key, value) in items {
            self.put(&key, &value);
        }
    }

    /// fetch K-V out of the committed datastore
    fn get(&mut self, key: &str) -> Option<String> {
        match self.store.get(key) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    fn has_entry(&mut self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// change the current MARF context to service reads from a different chain_tip
    ///   used to implement time-shifted evaluation.
    /// returns the previous block header hash on success
    fn set_block_hash(&mut self, bhh: StacksBlockId) -> Result<StacksBlockId> {
        self.chain_tip = bhh;
        Ok(bhh)
    }

    fn get_block_at_height(&mut self, height: u32) -> Option<StacksBlockId> {
        Some(self.chain_tip)
    }

    /// this function returns the current block height, as viewed by this marfed-kv structure,
    ///  i.e., it changes on time-shifted evaluation. the open_chain_tip functions always
    ///   return data about the chain tip that is currently open for writing.
    fn get_current_block_height(&mut self) -> u32 {
        self.chain_height.clone()
    }

    fn get_open_chain_tip_height(&mut self) -> u32 {
        self.chain_height.clone()
    }

    fn get_open_chain_tip(&mut self) -> StacksBlockId {
        self.chain_tip.clone()
    }

    /// The contract commitment is the hash of the contract, plus the block height in
    ///   which the contract was initialized.
    fn make_contract_commitment(&mut self, contract_hash: Sha512Trunc256Sum) -> String {
        "".to_string()
    }

    fn insert_metadata(&mut self, contract: &QualifiedContractIdentifier, key: &str, value: &str) {
        // let bhh = self.get_open_chain_tip();
        // self.get_side_store().insert_metadata(&bhh, &contract.to_string(), key, value)
        self.metadata
            .insert((contract.to_string(), key.to_string()), value.to_string());
    }

    fn get_metadata(
        &mut self,
        contract: &QualifiedContractIdentifier,
        key: &str,
    ) -> Result<Option<String>> {
        // let (bhh, _) = self.get_contract_hash(contract)?;
        // Ok(self.get_side_store().get_metadata(&bhh, &contract.to_string(), key))
        let key = &(contract.to_string(), key.to_string());

        match self.metadata.get(key) {
            Some(result) => Ok(Some(result.to_string())),
            None => Ok(None),
        }
    }
}

impl Datastore {
    pub fn open(path_str: &str, miner_tip: Option<&StacksBlockId>) -> Result<Datastore> {
        Ok(Datastore::new())
    }

    pub fn as_clarity_db<'a>(&'a mut self, headers_db: &'a dyn HeadersDB) -> ClarityDatabase<'a> {
        ClarityDatabase::new(self, headers_db)
    }

    pub fn as_analysis_db<'a>(&'a mut self) -> AnalysisDatabase<'a> {
        AnalysisDatabase::new(self)
    }

    /// begin, commit, rollback a save point identified by key
    ///    this is used to clean up any data from aborted blocks
    ///     (NOT aborted transactions that is handled by the clarity vm directly).
    /// The block header hash is used for identifying savepoints.
    ///     this _cannot_ be used to rollback to arbitrary prior block hash, because that
    ///     blockhash would already have committed and no longer exist in the save point stack.
    /// this is a "lower-level" rollback than the roll backs performed in
    ///   ClarityDatabase or AnalysisDatabase -- this is done at the backing store level.

    pub fn begin(&mut self, current: &StacksBlockId, next: &StacksBlockId) {
        // self.marf.begin(current, next)
        //     .expect(&format!("ERROR: Failed to begin new MARF block {} - {})", current, next));
        // self.chain_tip = self.marf.get_open_chain_tip()
        //     .expect("ERROR: Failed to get open MARF")
        //     .clone();
        // self.side_store.begin(&self.chain_tip);
    }
    pub fn rollback(&mut self) {
        // self.marf.drop_current();
        // self.side_store.rollback(&self.chain_tip);
        // self.chain_tip = StacksBlockId::sentinel();
    }
    // This is used by miners
    //   so that the block validation and processing logic doesn't
    //   reprocess the same data as if it were already loaded
    pub fn commit_mined_block(&mut self, will_move_to: &StacksBlockId) {
        // rollback the side_store
        //    the side_store shouldn't commit data for blocks that won't be
        //    included in the processed chainstate (like a block constructed during mining)
        //    _if_ for some reason, we do want to be able to access that mined chain state in the future,
        //    we should probably commit the data to a different table which does not have uniqueness constraints.
        // self.side_store.rollback(&self.chain_tip);
        // self.marf.commit_mined(will_move_to)
        //     .expect("ERROR: Failed to commit MARF block");
    }
    pub fn commit_to(&mut self, final_bhh: &StacksBlockId) {
        // println!("commit_to({})", final_bhh);
        // self.side_store.commit_metadata_to(&self.chain_tip, final_bhh);
        // self.side_store.commit(&self.chain_tip);
        // self.marf.commit_to(final_bhh)
        //     .expect("ERROR: Failed to commit MARF block");
    }
    pub fn get_chain_tip(&self) -> &StacksBlockId {
        &self.chain_tip
    }

    pub fn set_chain_tip(&mut self, bhh: &StacksBlockId) {
        self.chain_tip = bhh.clone();
    }

    pub fn put(&mut self, key: &str, value: &str) {
        // let marf_value = MARFValue::from_value(value);
        // self.side_store.put(&marf_value.to_hex(), value);

        // self.marf.insert(key, marf_value)
        //     .expect("ERROR: Unexpected MARF Failure")
        self.store.insert(key.to_string(), value.to_string());
    }

    pub fn make_contract_hash_key(contract: &QualifiedContractIdentifier) -> String {
        format!("clarity-contract::{}", contract)
    }
}

// impl ClarityBackingStore for MarfedKV {
//     fn get_side_store(&mut self) -> &mut SqliteConnection {
//         &mut self.side_store
//     }

//     fn set_block_hash(&mut self, bhh: StacksBlockId) -> Result<StacksBlockId> {
//         self.marf.check_ancestor_block_hash(&bhh).map_err(|e| {
//             match e {
//                 MarfError::NotFoundError => RuntimeErrorType::UnknownBlockHeaderHash(BlockHeaderHash(bhh.0)),
//                 MarfError::NonMatchingForks(_,_) => RuntimeErrorType::UnknownBlockHeaderHash(BlockHeaderHash(bhh.0)),
//                 _ => panic!("ERROR: Unexpected MARF failure: {}", e)
//             }
//         })?;

//         let result = Ok(self.chain_tip);
//         self.chain_tip = bhh;

//         result
//     }

//     fn get_current_block_height(&mut self) -> u32 {
//         self.marf.get_block_height_of(&self.chain_tip, &self.chain_tip)
//             .expect("Unexpected MARF failure.")
//             .expect("Failed to obtain current block height.")
//     }

//     fn get_block_at_height(&mut self, block_height: u32) -> Option<StacksBlockId> {
//         self.marf.get_bhh_at_height(&self.chain_tip, block_height)
//             .expect("Unexpected MARF failure.")
//             .map(|x| StacksBlockId(x.to_bytes()))
//     }

//     fn get_open_chain_tip(&mut self) -> StacksBlockId {
//         StacksBlockId(
//             self.marf.get_open_chain_tip()
//                 .expect("Attempted to get the open chain tip from an unopened context.")
//                 .clone()
//                 .to_bytes())
//     }

//     fn get_open_chain_tip_height(&mut self) -> u32 {
//         self.marf.get_open_chain_tip_height()
//             .expect("Attempted to get the open chain tip from an unopened context.")
//     }

//     fn get(&mut self, key: &str) -> Option<String> {
//         self.marf.get(&self.chain_tip, key)
//             .or_else(|e| {
//                 match e {
//                     MarfError::NotFoundError => Ok(None),
//                     _ => Err(e)
//                 }
//             })
//             .expect("ERROR: Unexpected MARF Failure on GET")
//             .map(|marf_value| {
//                 let side_key = marf_value.to_hex();
//                 self.side_store.get(&side_key)
//                     .expect(&format!("ERROR: MARF contained value_hash not found in side storage: {}",
//                                         side_key))
//             })
//     }

//     fn put_all(&mut self, mut items: Vec<(String, String)>) {
//         let mut keys = Vec::new();
//         let mut values = Vec::new();
//         for (key, value) in items.drain(..) {
//             let marf_value = MARFValue::from_value(&value);
//             self.side_store.put(&marf_value.to_hex(), &value);
//             keys.push(key);
//             values.push(marf_value);
//         }
//         self.marf.insert_batch(&keys, values)
//             .expect("ERROR: Unexpected MARF Failure");
//     }
// }
