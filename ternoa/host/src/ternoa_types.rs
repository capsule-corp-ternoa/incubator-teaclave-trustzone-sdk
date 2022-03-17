pub mod nft {
    pub type NFTId = u32;

    /// How NFT IDs are encoded. In the JSON Types this should be "Text" and not "Vec<8>".
    pub type NFTSeriesId = Vec<u8>;

    pub type AccountId = [u8; 32];

    pub type TextFormat = Vec<u8>;

    pub struct NFTData<AccountId> {
	// NFT owner
	pub owner: AccountId,
	// NFT creator
	pub creator: AccountId,
	// IPFS reference
	pub ipfs_reference: TextFormat,
	// Series ID
	pub series_id: NFTSeriesId,
	// Is listed for sale
	pub listed_for_sale: bool,
	// Is being transmitted
	pub is_in_transmission: bool,
	// Is NFT converted to capsule
	pub is_capsule: bool,
	// Is secret
	pub is_secret: bool,
	// Delegated
	pub is_delegated: bool,
	// Royalties fee
	pub royalties: u8,
}

}