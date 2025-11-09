pub struct Data{}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type _Context<'a> = poise::Context<'a, Data, Error>;
