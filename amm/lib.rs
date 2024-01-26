#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[allow(non_snake_case)]
#[ink::contract]
mod amm {
    use ink_storage::Mapping ;
    // const PRECISION: u32 = 1_00;

    #[ink(storage)]
    pub struct Amm {
        totalshare : u32 ,
        totaltoken1 : u32 ,
        totaltoken2 : u32 ,
        shares : Mapping< AccountId,u32> ,
        token1balance : Mapping<AccountId,u32> ,
        token2balance :Mapping <AccountId,u32> ,
        k : u32 , 
    }
    #[ink(impl)] 
    impl Amm 
    {
        pub fn activepool(&self) -> bool {
            if self.totalshare > 0 {
                return true ; 
            } else {
                return false ;
            }
        }
        pub fn validAmountcheck1(&self , deposit : u32 ) -> bool {
           let caller = self.env().caller();
            if deposit > 0 {
                if self.token1balance.get(&caller).unwrap_or(0) >= deposit {
                    true 
                } else {
                    false 
                }
            } else {
                return false 
            }
        }
        pub fn validAmountcheck2(&self , deposit : u32 ) -> bool {
            let caller = self.env().caller();
             if deposit > 0 {
                 if self.token2balance.get(&caller).unwrap_or(0) >= deposit {
                     true 
                 } else {
                     false 
                 }
             } else {
                 return false 
             }
         }  pub fn sharecheck(&self , deposit : u32 ) -> bool {
            let caller = self.env().caller();
             if deposit > 0 {
                 if self.token2balance.get(&caller).unwrap_or(0) >= deposit {
                     true 
                 } else {
                     false 
                 }
             } else {
                 return false 
             }
         }
        
    }

    impl Amm {
        #[ink(constructor)] 
        pub fn new() -> Self {
            // let caller = Self::env().caller() ;
            //new_mapp.insert(&caller, &0) ;
            Self {
                totalshare : 0 ,
                totaltoken1 : 0 ,
                totaltoken2 : 0 ,
                shares : Mapping::default(),
                token1balance : Mapping::default() ,
                token2balance : Mapping::default() ,
                 k : 0 ,
            }
        }
        #[ink(message)] 
        pub fn getTotalSupply1 (&self) -> u32 {
            self.totaltoken1 
        }
        #[ink(message)]
        pub fn getTotalSupply2(&self) -> u32 {
            self.totaltoken2 
        }
        #[ink(message)]
        pub fn getToken1balance(&self) ->u32 {
            let caller = self.env().caller() ;
            self.token1balance.get(&caller).unwrap_or(0) 
        }
        #[ink(message)]
        pub fn getToken2balance(&self) -> u32 {
            let caller = self.env().caller() ;
            self.token2balance.get(&caller).unwrap_or(0)
        }
        #[ink(message)]
        pub fn getshares(&self) -> u32 {
            let caller = self.env().caller() ;
            self.shares.get(&caller).unwrap_or(0)
        }
        #[ink(message)]
        pub fn getTotalShare(&self) -> u32 {
            self.totalshare
        }
        #[ink(message)] 
        pub fn faucet(&mut self , amt1 : u32  , amt2 : u32) -> bool {
            let caller = self.env().caller() ;
            self.token1balance.insert(&caller, &(amt1 +self.token1balance.get(&caller).unwrap_or(0) ));
            self.token2balance.insert(&caller , &(amt2 + self.token2balance.get(&caller).unwrap_or(0))); 
            true 
        }
        #[ink(message)]
        pub fn provide(&mut self, amt1 : u32 , amt2 :u32){
            assert!(self.validAmountcheck1(amt1));
            assert!(self.validAmountcheck2(amt2));
            let mut share : u32 ;
            if self.totalshare == 0  {
                share = 100 ;
            }
            else {
                let share1 = (self.getTotalShare()*amt1)/self.getTotalSupply1() ;
                let share2 = (self.getTotalShare()*amt2)/self.getTotalSupply2() ; 
                if share1 == share2 {
                    share = share1 ;
                } else {
                    share =  0 ;
                }
            }
            let caller = self.env().caller() ;
            let t1  = self.token1balance.get(&caller).unwrap_or(0) - amt1;
            let t2  = self.token2balance.get(&caller).unwrap_or(0) - amt2;
            self.token1balance.insert(&caller, &t1);
            self.token2balance.insert(&caller, &t2);
            self.totaltoken1 += amt1 ;
            self.totaltoken2 += amt2 ;
            self.k = self.totaltoken1*self.totaltoken2 ;
            self.shares.insert(&caller, &(share + self.shares.get(caller).unwrap_or(0))) ;
            self.totalshare += share ;
        }
        #[ink(message)]
        pub fn estimatToken(&self , share : u32 ) -> (u32 , u32) {

            let token1inpool = (share*self.totaltoken1)/self.getTotalShare() ;
            let token2inpool = (share*self.totaltoken2)/self.getTotalShare() ;
            (token1inpool , token2inpool)
        }
        #[ink(message)]
        pub fn withdraw(&mut self , _share : u32 )  {
            // assert!(self.sharecheck(_share), "not have the correct share ");
            // assert!(self.activepool() ," not the active pool ") ;

            let (amt1 ,amt2) = self.estimatToken(_share) ;
            let caller = self.env().caller() ;
            self.shares.insert(&caller,&(self.shares.get(caller).unwrap()-_share));
            
            // add the token to personal account 
            self.token1balance.insert(&caller, &(self.token1balance.get(&caller).unwrap() + amt1 ));
            self.token2balance.insert(&caller, &(self.token2balance.get(&caller).unwrap() + amt2 ));

            //total token substract 
            self.totaltoken1 -= amt1 ;
            self.totaltoken2 -= amt2 ;

            self.k = self.totaltoken1 * self.totaltoken2 ;
        }
        // write the swap function 
         #[ink(message)] 
        // how many t2 you get by swaping t1
        pub fn estimationt1(&self,amt1 : u32 ) -> u32{
        let t1after = self.totaltoken1 +amt1 ;
         let t2after = self.k/t1after ;
         let amt2 = self.totaltoken2-t2after ;
         amt2 
        }
        //how many t1 you want to get the amt2 amount of t2 token 
        #[ink(message)] 
        pub fn estoft1fort2(&self , amt2 : u32) -> u32 {
            assert!(amt2 < self.totaltoken2, "should be amt2 < self.totaltoken2" ) ;
            let amt2after = self.totaltoken2 - amt2  ;
            let amt1after = self.k / amt2after ;
                amt1after - self.totaltoken1
        }  



     #[ink(message)] 
     pub fn swap1(&mut self , amt1 : u32 ) {
        assert!(self.validAmountcheck1(amt1) , "swap1 ");
        assert!(self.activepool());
        let amt2 = self.estimationt1(amt1) ;
        let caller = self.env().caller() ;
        let t1_after =  self.token1balance.get(&caller).unwrap() - amt1 ;
        let t2_after = self.token2balance.get(&caller).unwrap() +amt2 ;
        self.totaltoken1 -= amt1 ;
        self.totaltoken2 += amt2 ;
        self.token1balance.insert(&caller, &t1_after);
        self.token2balance.insert(&caller, &t2_after);
     }
 
     #[ink(message)] 
     // how many t2 you get by swaping t1
     pub fn estimationt2(&self,amt2 : u32 ) -> u32{
     let t2after = self.totaltoken2 +amt2 ;
      let t1after = self.k/t2after ;
      let amt1 = self.totaltoken1-t1after ;
      amt1
     }
     //how many t1 you want to get the amt2 amount of t2 token 
     #[ink(message)] 
     pub fn estoft2fort1(&self , amt1 : u32) -> u32 {
         assert!(amt1 < self.totaltoken1, "should be amt1 < self.totaltoken1" ) ;
         let amt1after = self.totaltoken1 - amt1  ;
         let amt2after = self.k / amt1after ;
             amt2after - self.totaltoken2
     }  



  #[ink(message)] 
  pub fn swap2(&mut self , amt2 : u32 ) {
     assert!(self.validAmountcheck1(amt2) , "swap2 ");
     assert!(self.activepool());
     let amt1 = self.estimationt1(amt2) ;
     let caller = self.env().caller() ;
     let t1_after = self.token2balance.get(&caller).unwrap() +amt1 ;
     let t2_after =  self.token1balance.get(&caller).unwrap() - amt2 ;
     self.totaltoken1 -= amt2 ;
     self.totaltoken2 += amt1 ;
     self.token1balance.insert(&caller, &t1_after);
     self.token2balance.insert(&caller, &t2_after);
  }


    }

   
    use ink_env::{call, test};
    #[cfg(test)]
    mod tests {
        use super::* ;
        #[ink::test]
        fn testinit() {
            let cont = Amm::new() ;
            assert_eq!(cont.getTotalShare(), 0) ;

        }
        #[ink::test]
        fn faucettest() {
            let mut amm = Amm::new() ;
            amm.faucet(20, 30);
            assert_eq!(amm.getToken1balance() , 20) ;
           assert_eq!(amm.getToken2balance() , 30 ) ;
        }
        #[ink::test] 
        fn providetest() {
            // Create a new instance of your contract
            let mut my_contract = Amm::new();
    
            // Set up the environment with a caller account
            let accounts = test::default_accounts::<ink_env::DefaultEnvironment>();
            let caller = accounts.alice;
    
            let _test = test::run_test::<ink_env::DefaultEnvironment, _>(|_| {
                 my_contract.faucet(200, 300);
                  my_contract.provide(100, 100);
                assert_eq!(my_contract.getToken1balance(), 100);
                 assert_eq!(my_contract.getToken2balance(), 200);
                assert_eq!(my_contract.totalshare, 100);
                Ok(())
            });
           
        }

        #[ink::test]
        fn withdrawtest() {
            let mut my_contract = Amm::new();
    
            // Set up the environment with a caller account
            let accounts = test::default_accounts::<ink_env::DefaultEnvironment>();
            let caller = accounts.alice;
    
            let _test = test::run_test::<ink_env::DefaultEnvironment, _>(|_| {
                 my_contract.faucet(200, 300);
                  my_contract.provide(100, 100);
                assert_eq!(my_contract.getToken1balance(), 100);
                 assert_eq!(my_contract.getToken2balance(), 200);
                assert_eq!(my_contract.totalshare, 100);
                assert_eq!(my_contract.getshares() , 100);
                 my_contract.withdraw(50);
                 assert_eq!(my_contract.getToken1balance(), 150);
                 assert_eq!(my_contract.getToken2balance(), 250);
                assert_eq!(my_contract.getTotalSupply1(), 50) ;
                assert_eq!(my_contract.getTotalSupply2(), 50) ;
                Ok(())
            });
        }


        #[ink::test] 
        fn swapestimationtest() {
            let mut my_contract = Amm::new();
    
            // Set up the environment with a caller account
            let accounts = test::default_accounts::<ink_env::DefaultEnvironment>();
            let caller = accounts.alice;
    
            let _test = test::run_test::<ink_env::DefaultEnvironment, _>(|_| {
                 my_contract.faucet(200, 300);
                  my_contract.provide(100, 100);
                assert_eq!(my_contract.getToken1balance(), 100);
                 assert_eq!(my_contract.getToken2balance(), 200);
                assert_eq!(my_contract.totalshare, 100);
                assert_eq!(my_contract.getshares() , 100);
                //  my_contract.withdraw(50);
                //  assert_eq!(my_contract.getToken1balance(), 150);
                //  assert_eq!(my_contract.getToken2balance(), 250);
                // assert_eq!(my_contract.getTotalSupply1(), 50) ;
                // assert_eq!(my_contract.getTotalSupply2(), 50) ;
                // assert_eq!(my_contract.k , 2500);
                assert_eq!(my_contract.estimationt1(50) , 34);
                assert_eq!(my_contract.estoft1fort2(30)  , 42) ;
                assert_eq!(my_contract.estimationt1(50), 34);
               
                my_contract.swap1(50) ;
                assert_eq!(my_contract.getTotalSupply1(), 50);
                assert_eq!(my_contract.getTotalSupply2(), 134);

                Ok(())
            });
        }

}
}
