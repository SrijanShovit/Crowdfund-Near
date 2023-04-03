mod model;
mod utils;
use crate::{
    model::{Crowdfund, Donation},
    utils::{assert_self, assert_single_promise_success, AccountId, ONE_NEAR},
};

//efficient serialisation to save gas
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    Promise,
};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen, PromiseIndex};

#[near_bindgen]
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner: AccountId,
    crowdfunds: Vec<Crowdfund>,
    donations: Vec<Donation>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(owner: AccountId) -> Self {
        let crowdfunds: Vec<Crowdfund> = Vec::new();
        let donations: Vec<Donation> = Vec::new();

        Contract {
            owner,
            crowdfunds,
            donations,
        }
    }

    pub fn add_crowdfund(&mut self, title: String, donate: u128, description: String) {
        let id = self.crowdfunds.len() as i32;

        self.crowdfunds
            .push(Crowdfund::new(id, title, donate, description));

        env::log_str("Added a new crowdfund");
    }

    pub fn list_crowdfunds(&self) -> Vec<Crowdfund>{
        assert_self();
        let crowdfunds = &self.crowdfunds;
        return  crowdfunds.to_vec();
    }

    pub fn add_vote(&mut self,id:usize){
        let crowdfund:&mut Crowdfund = self.crowdfunds.get_mut(id).unwrap();
        let voter = env::predecessor_account_id().to_string();
        crowdfund.total_votes = crowdfund.total_votes + 1;
        env::log_str("vote submitted successfully");
        crowdfund.votes.push(voter);
    }

    pub fn add_donation(&mut self,id:usize,amount:u128){
        let transfer_amount:u128 = ONE_NEAR * amount;
        let crowdfund : &mut Crowdfund = self.crowdfunds.get_mut(id).unwrap();
        crowdfund.total_donations = crowdfund.total_donations + transfer_amount;
        self.donations.push(Donation::new());

        Promise::new(env::predecessor_account_id()).transfer(transfer_amount);
        env::log_str("You have donated successfulyy");
    }

    
    pub fn get_total_donations(&mut self,id:usize) -> u128{
        let crowdfund : &mut Crowdfund = self.crowdfunds.get_mut(id).unwrap();
        return crowdfund.total_donations;
    }
    
    pub fn crowdfund_count(&mut self)->usize{
        return self.crowdfunds.len();
    }
}
