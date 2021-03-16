
mod white;
mod yellow;
mod orange;
mod red;
mod pink;
mod purple;
mod blue;
mod olive;
mod green;
mod brown;
mod gray;
mod black;

pub use white::*;
pub use yellow::*;
pub use orange::*;
pub use red::*;
pub use pink::*;
pub use purple::*;
pub use blue::*;
pub use olive::*;
pub use green::*;
pub use brown::*;
pub use gray::*;
pub use black::*;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
