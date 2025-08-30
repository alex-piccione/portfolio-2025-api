
pub mod GetRecord {

    pub struct Request {
        pub id: u32
    }

    pub struct Response {
        pub id: u32,
        pub date: String,
        pub providerId: String,
        pub operation: String,
        pub currencyId: String,
        pub amount: f64,
    }
}

