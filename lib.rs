#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod incrementor {


    #[ink(storage)]
    pub struct Incrementer{
        value: i32,
    }


    #[ink(event)]
    pub struct Incremented{
        by: i32,
        new_value: i32,
        who: AccountId,
    }


    impl  Incrementer{


        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self { value: init_value }
        }



        #[ink(message)]
        pub fn incr(&mut self,by: i32){
            self.value += by;
            let signer = self.env().caller();
            self.env().emit_event(Incremented{by, new_value: self.value, who: signer})
        }

        #[ink(message)]
        pub fn get_value(&self) -> i32{
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_increment() {
            let mut incrementer = Incrementer::new(5);
            assert_eq!(5, incrementer.get_value());

            incrementer.incr(10);
            assert_eq!(15, incrementer.get_value());

            incrementer.incr(-5);
            assert_eq!(10, incrementer.get_value());
        }
    }
}