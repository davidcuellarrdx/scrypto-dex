use radix_engine_interface::prelude::*;
use scrypto::this_package;
use scrypto_test::prelude::*;
use scrypto_unit::*;
use scrypto::prelude::*;
use transaction::manifest::decompiler::ManifestObjectNames;

struct Account {
    account_component: ComponentAddress,
    public_key: Secp256k1PublicKey,
}

struct TestEnvironment {
    test_runner: DefaultTestRunner,
    account: Account,
    token_1: ResourceAddress,
    token_2: ResourceAddress,
    token_3: ResourceAddress,
    token_4: ResourceAddress,
    package_address: PackageAddress,
    component_address: ComponentAddress,
}

impl TestEnvironment {
    pub fn new() -> Self {
        let mut test_runner = TestRunnerBuilder::new().build();

        let (public_key, _private_key, account_component) = test_runner.new_allocated_account();

        let account = Account { account_component, public_key };

        let token_1 = test_runner.create_fungible_resource(dec!(2000000), 18, account_component);
        let token_2 = test_runner.create_fungible_resource(dec!(2000000), 18, account_component);
        let token_3 = test_runner.create_fungible_resource(dec!(2000000), 18, account_component);
        let token_4 = test_runner.create_fungible_resource(dec!(2000000), 18, account_component);

        let package_address = test_runner.compile_and_publish(this_package!());

        let manifest = ManifestBuilder::new()
            .call_function(
                package_address,
                "ScryptoDex",
                "instantiate_scrypto_dex",
                manifest_args!()
            )
            // .set_metadata(token_1, "name","Token1")
            // .set_metadata(token_1, "symbol","TK1")
            .build();

        let receipt = test_runner.execute_manifest_ignoring_fee(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&public_key)]
        );

        let component_address = receipt.expect_commit_success().new_component_addresses()[0];
        //let scrypto_nft = receipt.expect_commit_success().new_resource_addresses()[0];

        Self {
            test_runner,
            account,
            token_1,
            token_2,
            token_3,
            token_4,
            package_address,
            component_address,
        }
    }

    pub fn execute_manifest_ignoring_fee(
        &mut self,
        naming: ManifestObjectNames,
        manifest: TransactionManifestV1,
        name: &str
    ) -> TransactionReceipt {
        dump_manifest_to_file_system(
            naming,
            &manifest,
            "./manifests/tests",
            Some(name),
            &NetworkDefinition::mainnet()
        ).err();

        self.test_runner.execute_manifest_ignoring_fee(
            manifest,
            vec![NonFungibleGlobalId::from_public_key(&self.account.public_key)]
        )
    }

    pub fn instantiate_scrypto_dex(&mut self) -> TransactionReceipt {
        let manifest = ManifestBuilder::new().call_function(
            self.package_address,
            "ScryptoDex",
            "instantiate_scrypto_dex",
            manifest_args!()
        );
        // .set_metadata(self.token_1, "name","Token1")
        // .set_metadata(self.token_1, "symbol","TK1")

        self.execute_manifest_ignoring_fee(
            manifest.object_names(),
            manifest.build(),
            "instantiate_scrypto_dex"
        )
    }

    pub fn add_liquidity(
        &mut self,
        token_a: ResourceAddress,
        token_b: ResourceAddress,
        amount_a: Decimal,
        amount_b: Decimal
    ) -> TransactionReceipt {
        let manifest = ManifestBuilder::new()
            .lock_fee(self.account.account_component, dec!(1))
            .withdraw_from_account(self.account.account_component, token_a, amount_a)
            .take_from_worktop(token_a, amount_a, "token_a")
            .withdraw_from_account(self.account.account_component, token_b, amount_b)
            .take_from_worktop(token_b, amount_b, "token_b")
            .call_method_with_name_lookup(self.component_address, "add_liquidity", |lookup| (
                lookup.bucket("token_a"),
                lookup.bucket("token_b"),
            ))
            .deposit_batch(self.account.account_component);

        self.execute_manifest_ignoring_fee(
            manifest.object_names(),
            manifest.build(),
            "add_liquidity"
        )
    }

    pub fn swap(
        &mut self,
        token_a: ResourceAddress,
        token_b: ResourceAddress,
        amount_a: Decimal
    ) -> TransactionReceipt {
        let manifest = ManifestBuilder::new()
            .lock_fee(self.account.account_component, dec!(1))
            .withdraw_from_account(self.account.account_component, token_a, amount_a)
            .take_from_worktop(token_a, amount_a, "token_a")
            .call_method_with_name_lookup(self.component_address, "swap", |lookup| (
                lookup.bucket("token_a"),
                token_b,
            ))
            .deposit_batch(self.account.account_component);

        self.execute_manifest_ignoring_fee(manifest.object_names(), manifest.build(), "swap")
    }

    pub fn remove_liquidity(
        &mut self,
        token_a: ResourceAddress,
        amount_a: Decimal
    ) -> TransactionReceipt {
        let manifest = ManifestBuilder::new()
            .lock_fee(self.account.account_component, dec!(1))
            .withdraw_from_account(self.account.account_component, token_a, amount_a)
            .take_from_worktop(token_a, amount_a, "token_a")
            .call_method_with_name_lookup(self.component_address, "remove_liquidity", |lookup| (
                lookup.bucket("token_a"),
            ))
            .deposit_batch(self.account.account_component);

        self.execute_manifest_ignoring_fee(manifest.object_names(), manifest.build(), "swap")
    }

    //
}

#[test]
fn can_instantiate_scrypto_dex() {
    let mut test_env = TestEnvironment::new();

    let receipt = test_env.instantiate_scrypto_dex();

    receipt.expect_commit_success();

    println!("Transaction Receipt: {}", receipt.display(&AddressBech32Encoder::for_simulator()));
}

#[test]
fn can_initial_liquidity() {
    let mut test_env = TestEnvironment::new();

    let receipt = test_env.add_liquidity(
        test_env.token_1,
        test_env.token_2,
        dec!(2000),
        dec!(1000)
    );

    receipt.expect_commit_success();

    println!("Transaction Receipt: {}", receipt.display(&AddressBech32Encoder::for_simulator()));

    let receipt2 = test_env.add_liquidity(
        test_env.token_3,
        test_env.token_4,
        dec!(2000),
        dec!(1000)
    );

    receipt2.expect_commit_success();

    println!("Transaction Receipt: {}", receipt2.display(&AddressBech32Encoder::for_simulator()));
}

#[test]
fn can_swap() {
    let mut test_env = TestEnvironment::new();

    test_env.add_liquidity(test_env.token_1, test_env.token_2, dec!(2000), dec!(1000));

    let receipt = test_env.swap(test_env.token_1, test_env.token_2, dec!(20));

    receipt.expect_commit_success();

    println!("Transaction Receipt: {}", receipt.display(&AddressBech32Encoder::for_simulator()));
}

#[test]
fn can_remove_liquidity() {
    let mut test_env = TestEnvironment::new();

    let receipt =test_env.add_liquidity(test_env.token_1, test_env.token_2, dec!(2000), dec!(1000));

    let lp_token = receipt.expect_commit_success().new_resource_addresses();

    let receipt2 = test_env.remove_liquidity(lp_token[0], dec!(20));

    receipt2.expect_commit_success();

    println!("Transaction Receipt: {}", receipt2.display(&AddressBech32Encoder::for_simulator()));
}

// =========================================== FAIL SCENARIOS =========================================== //


#[test]
fn cannot_swap() {
    let mut test_env = TestEnvironment::new();

    test_env.add_liquidity(test_env.token_1, test_env.token_2, dec!(2000), dec!(1000));

    let receipt = test_env.swap(test_env.token_1, test_env.token_3, dec!(20));

    receipt.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::ApplicationError(ApplicationError::PanicMessage(..))
        )
    })
}


#[test]
fn cannot_remove_liquidity() {
    let mut test_env = TestEnvironment::new();

    let receipt =test_env.add_liquidity(test_env.token_1, test_env.token_2, dec!(2000), dec!(1000));

    receipt.expect_commit_success();

    let receipt2 = test_env.remove_liquidity(test_env.token_1, dec!(20));

    receipt2.expect_specific_failure(|e| {
        matches!(
            e,
            RuntimeError::ApplicationError(ApplicationError::PanicMessage(..))
        )
    })
}