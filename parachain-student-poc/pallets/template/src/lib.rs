#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use codec::alloc::string::ToString;
    use frame_support::sp_std::prelude::ToOwned;
    use frame_support::{
        dispatch::{DispatchResult, Vec},
        pallet_prelude::*,
    };
    use frame_system::pallet_prelude::*;
    use scale_info::prelude::string::String;

    #[pallet::storage]
    pub(super) type Students<T: Config> =
        StorageMap<_, Blake2_128Concat, u32, (T::AccountId, String)>;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        StudentCreated {
            student_id: u32,
            student_name: String,
        },
        StudentRetrieved {
            student_id: u32,
            student_name: String,
        },
        AllStudentsRetrieved {
            students: Vec<(u32, String)>,
        },
        MarksScored {
            student_id: u32,
            average_marks: u32,
            result: String,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        StudentNotFound,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        #[pallet::call_index(1)]
        pub fn create_student(origin: OriginFor<T>, student_name: String) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            // Get the next available student_id
            let student_id = Students::<T>::iter().map(|(key, _)| key).max().unwrap_or(0) + 1;

            // Store the student information
            Students::<T>::insert(&student_id, (&sender, student_name.clone()));

            // Emit an event that the student was created
            Self::deposit_event(Event::StudentCreated {
                student_id,
                student_name,
            });
            Ok(())
        }

        #[pallet::weight(0)]
        #[pallet::call_index(2)]
        pub fn get_student(origin: OriginFor<T>, student_id: Option<u32>) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            match student_id {
                Some(id) => {
                    // Retrieve the specific student
                    let student = Students::<T>::get(&id).ok_or(Error::<T>::StudentNotFound)?;
                    let student_name_vec = student.1;

                    Self::deposit_event(Event::StudentRetrieved {
                        student_id: id,
                        student_name: student_name_vec,
                    });
                    Ok(())
                }
                None => {
                    // Retrieve all students
                    let students = Students::<T>::iter()
                        .map(|(id, (_, name_fixed))| (id, name_fixed))
                        .collect();

                    Self::deposit_event(Event::AllStudentsRetrieved { students });
                    Ok(())
                }
            }
        }
        #[pallet::weight(0)]
        #[pallet::call_index(3)]
        pub fn add_marks(
            _origin: OriginFor<T>,
            student_id: u32,
            internal_marks_1: u32,
            internal_marks_2: u32,
            internal_marks_3: u32,
        ) -> DispatchResult {
            let _student = Students::<T>::get(&student_id).ok_or(Error::<T>::StudentNotFound)?;
            let average_marks = (internal_marks_1 + internal_marks_2 + internal_marks_3) / 3;

            let result = if average_marks >= 40 { "Pass" } else { "Fail" };

            Self::deposit_event(Event::MarksScored {
                student_id,
                average_marks,
                result: result.to_owned(),
            });
            Ok(())
        }
    }
}
