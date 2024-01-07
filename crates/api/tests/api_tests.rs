use actix_web::{test, web, App};
use api::data_parser::DistributionEntry;
use api::endpoints::{get_info, get_proof, status, verify_proof, VerifyProofQuery};
use api::AppState;
use indexed_merkle_tree::{
  hasher::{Hasher, KeccakHasher},
  tree::IndexedMerkleTree,
};
use std::sync::Arc;

fn create_test_data() -> web::Data<Arc<AppState>> {
  let test_data = vec![
    DistributionEntry {
      address: "alice".to_string(),
      amount: "100".to_string(),
    },
    DistributionEntry {
      address: "bob".to_string(),
      amount: "200".to_string(),
    },
  ];
  let total_amount = "300".to_string();
  let tree = IndexedMerkleTree::<DistributionEntry, KeccakHasher>::new(test_data, KeccakHasher);
  web::Data::new(Arc::new(AppState { tree, total_amount }))
}

#[actix_rt::test]
async fn test_status_endpoint() {
  let app = test::init_service(App::new().service(status)).await;
  let req = test::TestRequest::get().uri("/").to_request();
  let resp = test::call_service(&app, req).await;
  assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_get_info_endpoint() {
  let app_state = create_test_data();
  let app = test::init_service(App::new().app_data(app_state.clone()).service(get_info)).await;

  let req = test::TestRequest::get().uri("/info").to_request();
  let resp = test::call_service(&app, req).await;

  assert!(resp.status().is_success());

  let body = test::read_body(resp).await;
  let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

  let exp_root_hash = format!("0x{}", hex::encode(app_state.tree.root.hash));

  assert_eq!(json["total_amount"], "300");
  assert_eq!(json["root_hash"].as_str().unwrap(), exp_root_hash);
}

#[actix_rt::test]
async fn test_get_proof_endpoint() {
  let app_state = create_test_data();
  let app = test::init_service(App::new().app_data(app_state.clone()).service(get_proof)).await;

  let req = test::TestRequest::get()
    .uri("/proof?address=alice")
    .to_request();
  let resp = test::call_service(&app, req).await;

  assert!(resp.status().is_success());

  let body = test::read_body(resp).await;
  let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

  let sibling_hash = format!(
    "0x{}",
    hex::encode(app_state.tree.leaves.get(&(0, 1)).unwrap().hash)
  );

  assert_eq!(json["amount"], "100");
  assert_eq!(json["proof"][0], sibling_hash);
}

#[actix_rt::test]
async fn test_get_proof_invalid_address() {
  let app_state = create_test_data();
  let app = test::init_service(App::new().app_data(app_state.clone()).service(get_proof)).await;

  let req = test::TestRequest::get()
    .uri("/proof?address=charlie")
    .to_request();
  let resp = test::call_service(&app, req).await;

  assert!(resp.status().is_server_error());
}

#[actix_rt::test]
async fn test_verify_proof_endpoint() {
  let app_state = create_test_data();
  let app = test::init_service(
    App::new()
      .app_data(app_state.clone())
      .service(get_proof)
      .service(verify_proof),
  )
  .await;

  let proof = app_state
    .tree
    .get_proof(KeccakHasher.hash_leaf(&"alice".as_bytes()))
    .unwrap();

  let req = test::TestRequest::post()
    .uri("/verify")
    .set_json(VerifyProofQuery {
      address: proof.data.address,
      amount: proof.data.amount,
      proof: proof
        .proof
        .iter()
        .map(|h| format!("0x{}", hex::encode(h)))
        .collect(),
    })
    .to_request();
  let resp = test::call_service(&app, req).await;

  assert!(resp.status().is_success());

  let body = test::read_body(resp).await;
  let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

  assert_eq!(json["valid"], true);
}
