# Substrate Node Template 
This is a example of a pallet that can be added in the substrate node template.FRAME allows runtime developers to declare domain specific modules called "pallets". In this pallets we declared 2 callable function inside that are ```create_claim()``` and ```revoke_claim()``` first one will allow users to claim for an event that already exists and second one will revoke a claim that doesn't exists and also revoke attempt to claim owned by some another account.


Pallets compromised of number of blockchain primitives:
1. Storage
2. Dispatchables
3. Events
4. Errors
5. Config


### Working
Add this pallet in runtime Cargo.toml

``` pallet-template = { version = "4.0.0-dev", default-features = false, path = "../pallets/template" } ```