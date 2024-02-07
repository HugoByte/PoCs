#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
	pub trait TemplateRuntimeApi<AccountId>
	where AccountId: Clone + Codec + Send + sp_std::cmp::PartialEq + 'static,
	{
		fn get_providers() -> Vec<AccountId>;
		fn get_provider_enclaves(provider: AccountId) -> Vec<AccountId>;
	}
}
