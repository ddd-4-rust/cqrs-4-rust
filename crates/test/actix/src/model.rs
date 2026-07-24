//! Generator inputs and persisted person read model.

mod abstract_persons_view;
mod gen_person_created_event;
mod gen_person_id;
mod gen_person_name;
mod person_entity;

pub use abstract_persons_view::AbstractPersonsView;
pub use gen_person_created_event::GenPersonCreatedEvent;
pub use gen_person_id::GenPersonId;
pub use gen_person_name::GenPersonName;
pub use person_entity::PersonEntity;
