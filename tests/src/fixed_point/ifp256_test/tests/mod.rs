use fuels::prelude::{abigen, launch_provider_and_get_wallet};

abigen!(Script(
    name = "TestIfp256",
    abi = "src/fixed_point/ifp256_test/out/debug/ifp256_test-abi.json"
),);

mod success {

    use super::*;

    #[tokio::test]
    async fn runs_ifp256_test_script() {
        let path_to_bin = "src/fixed_point/ifp256_test/out/debug/ifp256_test.bin";

        let wallet = launch_provider_and_get_wallet().await;

        let instance = TestIfp256::new(wallet, path_to_bin);

        let _result = instance.main().call().await;
    }
}
