use anchor_lang::error_code;

#[error_code]
pub enum MarketPlaceError {
    #[msg("The name is longer than 32 bits")]
    NameTooLong,
}
