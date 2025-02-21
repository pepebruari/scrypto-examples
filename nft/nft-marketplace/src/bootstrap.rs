use scrypto::prelude::*;

#[blueprint]
mod bootstrap {
    /// This is a bootstrap struct which creates all of the resources which we need to use to test the NFT marketplace.
    struct Bootstrap {}

    impl Bootstrap {
        /// Creates a number of NFT collections used for testing of the NFT marketplace blueprints.
        pub fn bootstrap() -> Vec<Bucket> {
            // Creating the resources used for our non-fungible tokens
            let cars: Bucket = ResourceBuilder::new_uuid_non_fungible()
                .metadata("name", "Cars NFT")
                .metadata(
                    "description",
                    "An NFT of the fastest cars known to mankind!",
                )
                .metadata("symbol", "FAST")
                .mint_initial_supply([
                    Car {
                        name: "Camry".to_string(),
                        manufacturer: "Toyota".to_string(),
                    },
                    Car {
                        name: "Altima".to_string(),
                        manufacturer: "Nissan".to_string(),
                    },
                    Car {
                        // Any raptor lovers?
                        name: "Raptor".to_string(),
                        manufacturer: "Ford".to_string(),
                    },
                    Car {
                        name: "Yukon".to_string(),
                        manufacturer: "GMC".to_string(),
                    },
                ]);

            let phones: Bucket = ResourceBuilder::new_uuid_non_fungible()
                .metadata("name", "Phones NFT")
                .metadata(
                    "description",
                    "Do you really want me to describe to you what a phone is?",
                )
                .metadata("symbol", "PHONE")
                .mint_initial_supply([
                    Phone {
                        name: "iPhone".to_string(),
                        manufacturer: "Apple".to_string(),
                    },
                    Phone {
                        name: "Galaxy".to_string(),
                        manufacturer: "Samsung".to_string(),
                    },
                    Phone {
                        name: "Pixel".to_string(),
                        manufacturer: "Google".to_string(),
                    },
                    Phone {
                        name: "P50".to_string(),
                        manufacturer: "Huawei".to_string(),
                    },
                ]);

            let laptops: Bucket = ResourceBuilder::new_uuid_non_fungible()
                .metadata("name", "Laptops NFT")
                .metadata("description", "Do you really want me to describe to you what a laptop is? I'm a bit concerned...")
                .metadata("symbol", "LTOP")
                .mint_initial_supply([
                    Laptop {
                        name: "MacBook".to_string(),
                        manufacturer: "Apple".to_string()
                    },
                    Laptop {
                        name: "Thinkpad".to_string(),
                        manufacturer: "Lenovo".to_string()
                    },
                    Laptop {
                        name: "Surface".to_string(),
                        manufacturer: "Microsoft".to_string()
                    },
                    Laptop {
                        name: "Chromebook".to_string(),
                        manufacturer: "Google".to_string()
                    }
                ]);

            // With all of the NFTs created, we can now return the buckets of tokens
            return vec![cars, phones, laptops];
        }
    }
}

#[derive(NonFungibleData, ScryptoSbor)]
struct Car {
    name: String,
    manufacturer: String,
}

#[derive(NonFungibleData, ScryptoSbor)]
struct Phone {
    name: String,
    manufacturer: String,
}

#[derive(NonFungibleData, ScryptoSbor)]
struct Laptop {
    name: String,
    manufacturer: String,
}
