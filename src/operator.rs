use crate::token::Token;

pub trait Operator: Sized {
    fn from(token: &Token) -> Option<Self>;
    fn assoc(&self) -> u8;
    fn prec(&self) -> u8;
}
