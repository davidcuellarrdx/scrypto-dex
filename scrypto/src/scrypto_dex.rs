use scrypto::prelude::*;
use crate::utils::*;
use crate::liquidity_pool::liquidity_pool::LiquidityPool;
use crate::liquidity_pool::liquidity_pool::LiquidityPoolFunctions;

#[blueprint]
mod scryptodex_module {
    struct ScryptoDex {
        liquidity_pools: HashMap<(ResourceAddress, ResourceAddress), Global<LiquidityPool>>,
        tracking_token_address_pair_mapping: HashMap<
            ResourceAddress,
            (ResourceAddress, ResourceAddress)
        >,
    }

    impl ScryptoDex {
        pub fn instantiate_scrypto_dex() -> Global<ScryptoDex> {
            (Self {
                liquidity_pools: HashMap::new(),
                tracking_token_address_pair_mapping: HashMap::new(),
            })
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize()
        }

        pub fn pool_exists(&self, address1: ResourceAddress, address2: ResourceAddress) -> bool {
            let sorted_addresses: (ResourceAddress, ResourceAddress) = sort_addresses(
                address1,
                address2
            );
            return self.liquidity_pools.contains_key(&sorted_addresses);
        }

        pub fn assert_pool_exists(
            &self,
            address1: ResourceAddress,
            address2: ResourceAddress,
            label: String
        ) {
            assert!(
                self.pool_exists(address1, address2),
                "[{}]: No liquidity pool exists for the given address pair.",
                label
            );
        }

        pub fn assert_pool_doesnt_exists(
            &self,
            address1: ResourceAddress,
            address2: ResourceAddress,
            label: String
        ) {
            assert!(
                !self.pool_exists(address1, address2),
                "[{}]: A liquidity pool with the given address pair already exists.",
                label
            );
        }

        pub fn new_liquidity_pool(
            &mut self,
            token1: FungibleBucket,
            token2: FungibleBucket
        ) -> FungibleBucket {
            // Checking if a liquidity pool already exists between these two tokens
            self.assert_pool_doesnt_exists(
                token1.resource_address(),
                token2.resource_address(),
                String::from("New Liquidity Pool")
            );

            // Sorting the two buckets according to their resource addresses and creating a liquidity pool from these
            // two buckets.
            let (bucket1, bucket2): (FungibleBucket, FungibleBucket) = sort_buckets(token1, token2);
            let addresses: (ResourceAddress, ResourceAddress) = (
                bucket1.resource_address(),
                bucket2.resource_address(),
            );

            let (liquidity_pool, tracking_tokens) =
                Blueprint::<LiquidityPool>::instantiate_liquidity_pool(
                    bucket1,
                    bucket2,
                    dec!("0.3")
                );

            // Adding the liquidity pool to the hashmap of all liquidity pools
            self.liquidity_pools.insert(addresses, liquidity_pool);

            // Adding the resource address of the tracking tokens to the hashmap that maps the tracking tokens with
            // the address of their token pairs
            self.tracking_token_address_pair_mapping.insert(
                tracking_tokens.resource_address(),
                addresses
            );

            // Returning the tracking tokens back to the caller of this method (the initial liquidity provider).
            return tracking_tokens;
        }

        pub fn add_liquidity(
            &mut self,
            token1: FungibleBucket,
            token2: FungibleBucket
        ) -> (Option<FungibleBucket>, Option<FungibleBucket>, FungibleBucket) {
            // Sorting the two buckets of tokens passed to this method and getting the addresses of their resources.
            let (bucket1, bucket2): (FungibleBucket, FungibleBucket) = sort_buckets(token1, token2);
            let addresses: (ResourceAddress, ResourceAddress) = (
                bucket1.resource_address(),
                bucket2.resource_address(),
            );

            // Attempting to get the liquidity pool component associated with the provided address pair.
            let optional_liquidity_pool: Option<&Global<LiquidityPool>> = self.liquidity_pools.get(
                &addresses
            );
            match optional_liquidity_pool {
                Some(liquidity_pool) => {
                    // If it matches it means that the liquidity pool exists.
                    info!(
                        "[DEX Add Liquidity]: Pool for {:?} already exists. Adding liquidity directly.",
                        addresses
                    );
                    let returns: (
                        FungibleBucket,
                        FungibleBucket,
                        FungibleBucket,
                    ) = liquidity_pool.add_liquidity(bucket1, bucket2);
                    (Some(returns.0), Some(returns.1), returns.2)
                }
                None => {
                    info!(
                        "[DEX Add Liquidity]: Pool for {:?} doesn't exist. Creating a new one.",
                        addresses
                    );
                    (None, None, self.new_liquidity_pool(bucket1, bucket2))
                }
            }
        }

        pub fn remove_liquidity(
            &mut self,
            tracking_tokens: FungibleBucket
        ) -> (FungibleBucket, FungibleBucket) {
            // Check to make sure that the tracking tokens provided are indeed valid tracking tokens that belong to this
            // DEX.
            assert!(
                self.tracking_token_address_pair_mapping.contains_key(
                    &tracking_tokens.resource_address()
                ),
                "[DEX Remove Liquidity]: The tracking tokens given do not belong to this exchange."
            );

            info!(
                "[DEX Remove Liquidity]: Pool for {:?}. Remove liquidity directly.",
                tracking_tokens
            );

            // Getting the address pair associated with the resource address of the tracking tokens and then requesting
            // the removal of liquidity from the liquidity pool
            let addresses: (
                ResourceAddress,
                ResourceAddress,
            ) = self.tracking_token_address_pair_mapping[&tracking_tokens.resource_address()];
            return self.liquidity_pools[&addresses].remove_liquidity(tracking_tokens);
        }

        pub fn swap(
            &mut self,
            tokens: FungibleBucket,
            output_resource_address: ResourceAddress
        ) -> FungibleBucket {
            // Checking if there does exist a liquidity pool for the given pair of tokens
            self.assert_pool_exists(
                tokens.resource_address(),
                output_resource_address,
                String::from("DEX Swap")
            );

            // Sorting the two addresses passed, getting the associated liquidity pool and then performing the swap.
            let sorted_addresses: (ResourceAddress, ResourceAddress) = sort_addresses(
                tokens.resource_address(),
                output_resource_address
            );
            return self.liquidity_pools[&sorted_addresses].swap(tokens);
        }
    }
}
