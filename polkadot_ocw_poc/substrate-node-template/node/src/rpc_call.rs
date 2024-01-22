use jsonrpsee::{core::client::ClientT, rpc_params};

pub async fn rpc_call_for_authorized_node(
	provider_url: &str,
	account: String,
	request_id: u64,
) -> Result<(), sc_service::Error> {
	let client = jsonrpsee::http_client::HttpClientBuilder::default()
		.build(provider_url)
		.unwrap();

	let params = rpc_params![account, request_id];
	let _res: String = client
		.request("template_authorizeNode", params)
		.await
		.map_err(|e| sc_service::Error::Other(e.to_string()))?;

	Ok(())
}
