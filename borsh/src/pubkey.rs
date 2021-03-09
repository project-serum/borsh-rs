use crate::maybestd::collections::HashMap;
use crate::maybestd::io;
use crate::maybestd::string::ToString;
use crate::schema::{Declaration, Definition};
use crate::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::pubkey::Pubkey;

impl BorshSerialize for Pubkey {
    fn serialize<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        self.to_bytes().serialize(writer)
    }
}

impl BorshDeserialize for Pubkey {
    fn deserialize(buf: &mut &[u8]) -> io::Result<Self> {
        let arr = BorshDeserialize::deserialize(buf)?;
        Ok(Pubkey::new_from_array(arr))
    }
}

impl BorshSchema for Pubkey {
    fn add_definitions_recursively(_definitions: &mut HashMap<Declaration, Definition>) {}

    fn declaration() -> Declaration {
        "pubkey".to_string()
    }
}
