mod address;
mod boolean;
mod company;
mod internet;
mod lorem;
mod name;
mod number;
mod phone_number;
#[cfg(feature = "chrono")]
mod time;

pub use self::address::Address;
pub use self::boolean::Boolean;
pub use self::company::Company;
pub use self::internet::Internet;
pub use self::lorem::Lorem;
pub use self::name::Name;
pub use self::number::Number;
pub use self::phone_number::PhoneNumber;
#[cfg(feature = "chrono")]
pub use self::time::Chrono;
pub use Faker;
