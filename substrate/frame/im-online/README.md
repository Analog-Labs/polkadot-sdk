# I'm online Module

If the local node is a validator (i.e. contains an authority key), this module
gossips a heartbeat transaction with each new session. The heartbeat functions
as a simple mechanism to signal that the node is online in the current era.

Received heartbeats are tracked for one era and reset with each new era. The
module exposes two public functions to query if a heartbeat has been received
in the current era or session.

The heartbeat is a signed transaction, which was signed using the session key
and includes the recent best block number of the local validators chain as well
as the `NetworkState`.
It is submitted as an Unsigned Transaction via off-chain workers.

- [`im_online::Config`](https://docs.rs/pallet-im-online/latest/pallet_im_online/trait.Config.html)
- [`Call`](https://docs.rs/pallet-im-online/latest/pallet_im_online/enum.Call.html)
- [`Module`](https://docs.rs/pallet-im-online/latest/pallet_im_online/struct.Module.html)

## Interface

### Public Functions

- `is_online` - True if the validator sent a heartbeat in the current session.

## Usage

```rust
use pallet_im_online::{self as im_online};

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + im_online::Config {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn is_online(origin: OriginFor<T>, authority_index: u32) -> DispatchResult {
            let _sender = ensure_signed(origin)?;
            let _is_online = <im_online::Pallet<T>>::is_online(authority_index);
            Ok(())
        }
    }
}
```

## Dependencies

This module depends on the [Session module](https://docs.rs/pallet-session/latest/pallet_session/).

License: Apache-2.0


## Release

Polkadot SDK Stable 2412
