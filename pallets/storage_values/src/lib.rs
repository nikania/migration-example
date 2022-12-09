#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config {
      /// The overarching event type.
      type RuntimeEvent: From<Event<Self, I>>
        + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

  #[pallet::storage]
  #[pallet::getter(fn value)]
  pub type Values<T: Config<I>, I: 'static = ()> = StorageMap<
    _, 
    Blake2_128Concat, T::AccountId, 
    i32, 
    ValueQuery
  >;

  #[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
    /// [something, who]
		ValueStored { value: i32, who: T::AccountId },
	}

  #[pallet::call]
  impl<T: Config<I>, I: 'static> Pallet<T, I> {

    #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
    pub fn store(origin: OriginFor<T>, value: i32) -> DispatchResult {
      let who = ensure_signed(origin)?;

      <Values<T, I>>::set(who.clone(), value);

      Self::deposit_event(Event::ValueStored { value, who });

      Ok(())
    }
  }
}