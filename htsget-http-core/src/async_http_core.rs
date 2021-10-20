use std::collections::HashMap;
use std::sync::Arc;

use futures::stream::FuturesUnordered;
use futures::StreamExt;
use tokio::select;

use htsget_search::htsget::{HtsGet, Response};

use crate::{
  convert_to_query, match_endpoints_get_request, match_endpoints_post_request, merge_responses,
  Endpoint, HtsGetError, JsonResponse, PostRequest, Result,
};

/// Gets a JSON response for a GET request. The GET request parameters must
/// be in a HashMap. The "id" field is the only mandatory one. The rest can be
/// consulted [here](https://samtools.github.io/hts-specs/htsget.html)
pub async fn get_response_for_get_request(
  searcher: Arc<impl HtsGet + Send + Sync + 'static>,
  mut query_information: HashMap<String, String>,
  endpoint: Endpoint,
) -> Result<JsonResponse> {
  match_endpoints_get_request(&endpoint, &mut query_information)?;
  let query = convert_to_query(&query_information)?;
  searcher
    .search(query)
    .await
    .map_err(|error| error.into())
    .map(JsonResponse::from_response)
}

/// Gets a response in JSON for a POST request.
/// The parameters can be consulted [here](https://samtools.github.io/hts-specs/htsget.html)
pub async fn get_response_for_post_request(
  searcher: Arc<impl HtsGet + Send + Sync + 'static>,
  mut request: PostRequest,
  id: impl Into<String>,
  endpoint: Endpoint,
) -> Result<JsonResponse> {
  match_endpoints_post_request(&endpoint, &mut request)?;

  let mut futures = FuturesUnordered::new();
  for query in request.get_queries(id)? {
    let owned_searcher = searcher.clone();
    futures.push(tokio::spawn(
      async move { owned_searcher.search(query).await },
    ));
  }
  let mut responses: Vec<Response> = Vec::new();
  loop {
    select! {
      Some(next) = futures.next() => responses.push(next.map_err(|err| HtsGetError::ConcurrencyError(err.to_string()))?.map_err(HtsGetError::from)?),
      else => break
    }
  }

  Ok(JsonResponse::from_response(
    // It's okay to unwrap because there will be at least one response
    merge_responses(responses).unwrap(),
  ))
}