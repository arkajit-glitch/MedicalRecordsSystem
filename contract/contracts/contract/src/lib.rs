#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct MedicalRecord {
    pub name: String,
    pub age: u32,
    pub diagnosis: String,
    pub treatment: String,
}

#[contract]
pub struct MedicalContract;

#[contractimpl]
impl MedicalContract {

    // Add or update medical record
    pub fn set_record(env: Env, patient: Address, record: MedicalRecord) {
        // Require patient authorization
        patient.require_auth();

        let key = symbol_short!("RECORD");

        env.storage().instance().set(&(key, patient), &record);
    }

    // Get medical record
    pub fn get_record(env: Env, patient: Address) -> Option<MedicalRecord> {
        let key = symbol_short!("RECORD");

        env.storage().instance().get(&(key, patient))
    }

    // Delete record (optional)
    pub fn delete_record(env: Env, patient: Address) {
        patient.require_auth();

        let key = symbol_short!("RECORD");

        env.storage().instance().remove(&(key, patient));
    }
}