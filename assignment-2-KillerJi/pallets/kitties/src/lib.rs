#![cfg_attr(not(feature = "std"),no_std)]

use codec::{Encode,Decode};
use frame_support::{decl_module,decl_storage,decl_event,
StorageValue,StorageDoubleMap,traits::Randomness,RuntimeDebug,
};
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;

pub trait Trait:frame_system::Trait{
    type Event:From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

}

#[derive(Encode,Decode,Clone,RuntimeDebug,PartialEq,Eq)]
pub struct Kitty(pub [u8;16]);//128 bit dna

decl_storage!{
    trait Store for Module<T: Trait> as KittyStorage {
        pub Kitties get(fn kitties):double_map hasher(blake2_128_concat)T::AccountId,hasher(blake2_128_concat)u32=>Option<Kitty>;
        pub NextKittyId get(fn next_kitty_id):u32 = 0;       
    }

}

decl_event!{
    pub enum Event<T>where
        <T as frame_system::Trait>::AccountId,
        {
            KittyCreated(AccountId,u32,Kitty),
        }
}



decl_module!{
    
    pub struct Module<T:Trait> for enum Call where origin:T::Origin{
    
		fn deposit_event() = default;

		#[weight = 0]
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;

			let payload = (
				<pallet_randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
				&sender,
				<frame_system::Module<T>>::extrinsic_index(),
			);
			let dna = payload.using_encoded(blake2_128);

			let kitty = Kitty(dna);
			let kitty_id = Self::next_kitty_id();
			<Kitties<T>>::insert(&sender, kitty_id, kitty.clone());
			NextKittyId::put(kitty_id + 1);


			Self::deposit_event(RawEvent::KittyCreated(sender, kitty_id, kitty))
		}
	

    }
}