use scrypto::prelude::*;
use crate::utils::*;

#[blueprint]
mod liquidity_pool {

    enable_method_auth! {
        roles {
            withdrawer => updatable_by: [OWNER, SELF];
        },
        methods {
            addresses => PUBLIC;
            other_resource_address => PUBLIC;
            swap => PUBLIC;
            add_liquidity => PUBLIC;
            remove_liquidity_logic => restrict_to: [SELF, withdrawer];
            remove_liquidity => PUBLIC;
        }
    }

    struct LiquidityPool {
        vaults: HashMap<ResourceAddress, FungibleVault>,
        pool_units_resource_manager: ResourceManager,
        fee: Decimal,
    }

    impl LiquidityPool {

        pub fn instantiate_liquidity_pool(
            bucket_a: FungibleBucket,
            bucket_b: FungibleBucket,
            fee: Decimal,
        ) -> (Global<LiquidityPool>, FungibleBucket){

            
            assert!(!bucket_a.is_empty() && !bucket_b.is_empty(), "You must pass in an initial supply of each token");

            assert!(fee >= dec!(0) && fee <= dec!(1), "Fee must be between 0 and 1");

            let (address_reservation, component_address) = 
                Runtime::allocate_component_address(LiquidityPool::blueprint_id());

            // Sorting the buckets and then creating the hashmap of the vaults from the sorted buckets
            let (bucket_a, bucket_b): (FungibleBucket, FungibleBucket) = sort_buckets(bucket_a, bucket_b);
            let addresses: (ResourceAddress, ResourceAddress) = (bucket_a.resource_address(), bucket_b.resource_address());

            let lp_id: String = format!("{:?}-{:?}", addresses.0, addresses.1);

            let pair_name: String = address_pair_symbol(addresses.0, addresses.1);

            info!(
                "[Pool Creation]: Creating new pool between tokens: {}, of name: {}, Ratio: {}:{}", 
                lp_id, pair_name, bucket_a.amount(), bucket_b.amount()
            );

            let mut vaults: HashMap<ResourceAddress, FungibleVault> = HashMap::new();
            vaults.insert(bucket_a.resource_address(), FungibleVault::with_bucket(bucket_a));
            vaults.insert(bucket_b.resource_address(), FungibleVault::with_bucket(bucket_b));

            let pool_units = ResourceBuilder::new_fungible(OwnerRole::None)
                .metadata(metadata!(
                    init {
                        "name" => format!("{} LP Tracking Token", pair_name), locked;
                        "symbol" => format!("LP-{}", pair_name), locked;
                        "lp_id" => format!("{}", lp_id), locked;
                    }
                ))
                .mint_roles(mint_roles!(
                    minter => rule!(require(global_caller(component_address)));
                    minter_updater => rule!(deny_all);
                ))
                .burn_roles(burn_roles!(
                    burner => rule!(require(global_caller(component_address)));
                    burner_updater => rule!(deny_all);
                ))
                .mint_initial_supply(100);

            

            let liquidity_pool = Self{
                vaults: vaults,
                pool_units_resource_manager: pool_units.resource_manager(),
                fee: fee,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .roles(roles!( 
                withdrawer => rule!(require(global_caller(component_address)));
            ))
            .with_address(address_reservation)
            .globalize();

            (liquidity_pool, pool_units)
        }

        pub fn addresses(&self) -> Vec<ResourceAddress> {
            return self.vaults.keys().cloned().collect::<Vec<ResourceAddress>>();
        }

        pub fn other_resource_address(
            &self,
            resource_address: ResourceAddress
        ) -> ResourceAddress {
            let addresses: Vec<ResourceAddress> = self.addresses();
            return if addresses[0] == resource_address {addresses[1]} else {addresses[0]};
        }

        pub fn swap(&mut self, input_tokens: FungibleBucket) -> FungibleBucket{

            let output_resource_address = self.other_resource_address(input_tokens.resource_address());

            let input_tokens_vault_amount =self.vaults[&input_tokens.resource_address()].amount();
            let output_tokens_vault_amount =self.vaults[&output_resource_address].amount();
            

            // Calculate the output amount of tokens based on the input amount 
            // and the pool fees
            let output_amount: Decimal = (output_tokens_vault_amount
            * (dec!("1") - self.fee)
            * input_tokens.amount())
            / (input_tokens_vault_amount + input_tokens.amount() 
            * (dec!("1") - self.fee));

            
            // Perform the swapping operation
            self.vaults.get_mut(&input_tokens.resource_address()).unwrap().put(input_tokens);

            self.vaults.get_mut(&output_resource_address).unwrap().take(output_amount)

            
        }
        
        pub fn add_liquidity(
            &mut self,
            token1: FungibleBucket,
            token2: FungibleBucket,
        ) -> (FungibleBucket, FungibleBucket, FungibleBucket) {

        
            // Getting the values of `dm` and `dn` based on the sorted buckets

            let (mut bucket_a, mut bucket_b): (FungibleBucket, FungibleBucket) = sort_buckets(token1, token2);

            let dm: Decimal = bucket_a.amount();
            let dn: Decimal = bucket_b.amount();
        
            // Getting the values of m and n from the liquidity pool vaults
            let m: Decimal = self.vaults[&bucket_a.resource_address()].amount();
            let n: Decimal = self.vaults[&bucket_b.resource_address()].amount();
        
            // Calculate the amount of tokens which will be added to each one of 
            //the vaults
            let (amount_a, amount_b): (Decimal, Decimal) =
                if ((m == Decimal::zero()) | (n == Decimal::zero())) 
                    | ((m / n) == (dm / dn)) 
                {
                    // Case 1
                    (dm, dn)
                } else if (m / n) < (dm / dn) {
                    // Case 2
                    (dn * m / n, dn)
                } else {
                    // Case 3
                    (dm, dm * n / m)
                };
        
            // Depositing the amount of tokens calculated into the liquidity pool
        
            self.vaults.get_mut(&bucket_a.resource_address()).unwrap().put(bucket_a.take(amount_a));
            self.vaults.get_mut(&bucket_b.resource_address()).unwrap().put(bucket_b.take(amount_b));

        
            // Mint pool units tokens to the liquidity provider
            let pool_units_amount: Decimal =
                if self.pool_units_resource_manager.total_supply().unwrap() == Decimal::zero() {
                    dec!("100.00")
                } else {
                    amount_a * self.pool_units_resource_manager.total_supply().unwrap() / m
                };
            let pool_units: FungibleBucket = self.pool_units_resource_manager.mint(pool_units_amount).as_fungible();
        
            // Return the remaining tokens to the caller as well as the pool units 
            // tokens
            (bucket_a, bucket_b, pool_units)
        }

        pub fn remove_liquidity_logic(&mut self, 
            address1: ResourceAddress, 
            address2: ResourceAddress, 
            bucket1_amount: Decimal, 
            bucket2_amount: Decimal ) -> (FungibleBucket, FungibleBucket)
        {
            
                    
            (
            self.vaults.get_mut(&address1).unwrap().take(bucket1_amount),
            self.vaults.get_mut(&address2).unwrap().take(bucket2_amount)
            )
        }

        pub fn remove_liquidity(&mut self, pool_units: FungibleBucket) -> (FungibleBucket, FungibleBucket) {
    
            assert!(
                pool_units.resource_address() == self.pool_units_resource_manager.address(),
                "Wrong token type passed in"
            );
        
            // Calculate the share based on the input LP tokens.
            let share = pool_units.amount() / 
                self.pool_units_resource_manager.total_supply().unwrap();
        
        
            // Burn the LP tokens received
            pool_units.burn();

            // Withdrawing the amount of tokens owed to this liquidity provider
            let addresses: Vec<ResourceAddress> = self.addresses();

            let bucket1_amount = self.vaults.get_mut(&addresses[0]).unwrap().amount() * share;
            let bucket2_amount = self.vaults.get_mut(&addresses[1]).unwrap().amount() * share;
        
            // Return the withdrawn tokens
            // (
            //     self.vaults.get_mut(&addresses[0]).unwrap().take(bucket1_amount),
            //     self.vaults.get_mut(&addresses[1]).unwrap().take(bucket2_amount),
            // )
            self.remove_liquidity_logic(addresses[0], addresses[1], bucket1_amount, bucket2_amount)
        }

    }
}
