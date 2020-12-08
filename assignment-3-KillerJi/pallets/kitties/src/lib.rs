#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{
	decl_module, decl_storage, decl_event, decl_error, StorageDoubleMap,
	traits::Randomness, RuntimeDebug,
};
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub enum Gender{
	Male,
	Female,
}
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
pub struct Kitty{
	dna :[u8;16],
	gender: Option<Gender>,

}

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as Kitties {
		/// Stores all the kitties, key is the kitty id
		pub Kitties get(fn kitties): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => Option<Kitty>;
		/// Stores the next kitty ID
		pub NextKittyId get(fn next_kitty_id): map hasher(blake2_128_concat) T::AccountId => u32;
	}
}

decl_event!{
	pub enum Event<T> where
		<T as frame_system::Trait>::AccountId,
	{
		/// A kitty is created. \[owner, kitty_id, kitty\]
		KittyCreated(AccountId, u32, Kitty),
		KittyBreed(AccountId, u32, Kitty),
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		KittiesIdOverflow,
		KittiesNonExistent,
		KittiesParentSexChooseError,
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		/// Create a new kitty
		#[weight = 1000]
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;
			let mut dna_count = 0;
			
			//let mut kitty_gender : Option<Gender> = Option::None;
			let kitty_gender;
			// TODO: ensure kitty id does not overflow			
			// return Err(Error::<T>::KittiesIdOverflow.into());
			let kitty_id = Self::next_kitty_id(&sender);
			frame_support::ensure!( ((kitty_id+1) < 4294967295), Error::<T>::KittiesIdOverflow);
			// Generate a random 128bit value
			let payload = (	
				<pallet_randomness_collective_flip::Module<T> as Randomness<T::Hash>>::random_seed(),
				&sender,
				<frame_system::Module<T>>::extrinsic_index(),
			);
			let dna = payload.using_encoded(blake2_128);
		
			for dna_son in dna.iter()
			{
				if dna_son % 2 == 0
					{dna_count += 1;}
			}
	
			if dna_count % 2 == 0
			{

				kitty_gender = Some(Gender::Female);
			}	
			else{

				kitty_gender = Some(Gender::Male);
			}	
			let kitty = Kitty{dna:dna,gender:kitty_gender,};
	
			<Kitties<T>>::insert(&sender,kitty_id,kitty.clone());

			<NextKittyId<T>>::insert(&sender,kitty_id + 1);
			// Emit event
			Self::deposit_event(RawEvent::KittyCreated(sender, kitty_id, kitty))
		}
		#[weight = 1000]
		pub fn breed(origin,kitty_one_id:u32,kitty_two_id:u32){
			let sender = ensure_signed(origin)?;
			frame_support::ensure!(<Kitties<T>>::get(&sender,kitty_one_id) != None, Error::<T>::KittiesNonExistent);
			frame_support::ensure!(<Kitties<T>>::get(&sender,kitty_two_id) != None, Error::<T>::KittiesNonExistent);
			let kitty_one_option = <Kitties<T>>::get(&sender,kitty_one_id);
			let kitty_two_option = <Kitties<T>>::get(&sender,kitty_two_id);
			let kitty_one = kitty_one_option.unwrap();
			let kitty_two = kitty_two_option.unwrap();

			let kitty_id = Self::next_kitty_id(&sender);// 0 2 
			frame_support::ensure!( ((kitty_id+1) < 4294967295), Error::<T>::KittiesIdOverflow);
			frame_support::ensure!( ( (kitty_one.gender == Some(Gender::Female))&&(kitty_two.gender == Some(Gender::Male))  ) || ( (kitty_one.gender == Some(Gender::Male))&&(kitty_two.gender == Some(Gender::Female)) ),
			Error::<T>::KittiesParentSexChooseError);
			let payload = (kitty_one.dna,kitty_two.dna,kitty_id);
			let kid_dna = payload.using_encoded(blake2_128);
			let mut dna_count = 0;
			
			//let mut kid_kitty_gender : Option<Gender> = Option::None;
			let kid_kitty_gender;
			for dna_son in kid_dna.iter()
			{
				if dna_son % 2 == 0
					{dna_count += 1;}
			}
	
			if dna_count % 2 == 0
			{
				kid_kitty_gender = Some(Gender::Female);
			}	
			else{
				kid_kitty_gender = Some(Gender::Male);
			}	
			let kid_kitty = Kitty{dna:kid_dna,gender:kid_kitty_gender};
			<Kitties<T>>::insert(&sender,kitty_id,kid_kitty.clone());
			<NextKittyId<T>>::insert(&sender,kitty_id + 1);
			// Emit event
			Self::deposit_event(RawEvent::KittyBreed(sender, kitty_id, kid_kitty))
		}
	}
		
}