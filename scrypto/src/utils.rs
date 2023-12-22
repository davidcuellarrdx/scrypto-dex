use scrypto::prelude::*;

pub fn sort_addresses(
    address1: ResourceAddress,
    address2: ResourceAddress
) -> (ResourceAddress, ResourceAddress) {
    return if address1.to_vec() > address2.to_vec() {
        (address1, address2)
    } else {
        (address2, address1)
    };
}

pub fn sort_buckets(
    bucket1: FungibleBucket,
    bucket2: FungibleBucket
) -> (FungibleBucket, FungibleBucket) {
    // Getting the sorted addresses of the two buckets given
    let sorted_addresses: (ResourceAddress, ResourceAddress) = sort_addresses(
        bucket1.resource_address(),
        bucket2.resource_address()
    );

    // Sorting the buckets and returning them back
    return if bucket1.resource_address() == sorted_addresses.0 {
        (bucket1, bucket2)
    } else {
        (bucket2, bucket1)
    };
}

pub fn address_pair_symbol(address1: ResourceAddress, address2: ResourceAddress) -> String {
    let addresses: (ResourceAddress, ResourceAddress) = sort_addresses(address1, address2);

    let symbol_a: String;
    if
        let Some(metadata) = ResourceManager::from(addresses.0)
            .get_metadata::<&str, _>("symbol")
            .unwrap()
    {
        symbol_a = metadata;
    } else {
        symbol_a = get_last_5_words(Runtime::bech32_encode_address(address1).as_str());
    }

    let symbol_b: String;
    if
        let Some(metadata) = ResourceManager::from(addresses.1)
            .get_metadata::<&str, _>("symbol")
            .unwrap()
    {
        symbol_b = metadata;
    } else {
        symbol_b = get_last_5_words(Runtime::bech32_encode_address(address2).as_str());
    }

    return format!("{}-{}", symbol_a, symbol_b);
}

fn get_last_5_words(input: &str) -> String {
    input
        .chars()
        .skip(input.len() - 5)
        .collect()
}
