pub mod update {
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Request {
        pub currency_id: i32,
    }
}
