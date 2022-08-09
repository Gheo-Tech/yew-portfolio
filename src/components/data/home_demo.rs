use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

const DEPOSIT: &str = "deposit";
const WITHDRAWAL: &str = "withdrawal";
const DISPUTE: &str = "dispute";
const RESOLVE: &str = "resolve";
const CHARGEBACK: &str = "chargeback";

type ClientID = u16;
type TxID = u32;
type Closed = bool;
type TxResult = Result<(), TxError>;

// This is a transaction.
#[derive(Debug, Deserialize, Clone)]
struct Tx {
    #[serde(rename = "type")]
    type_: TxType,
    client: ClientID,
    tx: TxID,
    amount: Option<Decimal>,
}

#[derive(Debug)]
enum TxError {
    AccountLocked,
    BadFormat,
    DisputeNotFound,
    DuplicateTx,
    InsufficientFunds,
    TxIsNotCredit,
    TxNotFound,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(try_from = "String")]
enum TxType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

struct TxFromStrError(String);
impl fmt::Display for TxFromStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse transaction type: {}", self.0)
    }
}

impl TryFrom<String> for TxType {
    type Error = TxFromStrError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            DEPOSIT => Ok(TxType::Deposit),
            WITHDRAWAL => Ok(TxType::Withdrawal),
            DISPUTE => Ok(TxType::Dispute),
            RESOLVE => Ok(TxType::Resolve),
            CHARGEBACK => Ok(TxType::Chargeback),
            _ => Err(TxFromStrError(s)),
        }
    }
}

#[derive(Debug)]
struct Account {
    available: Decimal,
    held: Decimal,
    locked: bool,
}

impl Account {
    fn new() -> Self {
        Account {
            available: Decimal::new(0, 0),
            held: Decimal::new(0, 0),
            locked: false,
        }
    }
    fn is_locked(&self) -> TxResult {
        if self.locked {
            return Err(TxError::AccountLocked);
        }
        Ok(())
    }
    fn credit(&mut self, amount: Decimal) -> TxResult {
        self.is_locked()?;
        self.available += amount;
        Ok(())
    }
    fn debit(&mut self, amount: Decimal) -> TxResult {
        self.is_locked()?;
        if self.available < amount {
            return Err(TxError::InsufficientFunds);
        }
        self.available -= amount;
        Ok(())
    }
    fn dispute(&mut self, amount: Decimal) -> TxResult {
        self.is_locked()?;
        if self.available < amount {
            return Err(TxError::InsufficientFunds);
        }
        self.available -= amount;
        self.held += amount;
        Ok(())
    }
    fn resolve(&mut self, amount: Decimal) -> TxResult {
        self.is_locked()?;
        self.held -= amount;
        self.available += amount;
        Ok(())
    }
    fn chargeback(&mut self, amount: Decimal) -> TxResult {
        self.is_locked()?;
        self.held -= amount;
        self.locked = true;
        Ok(())
    }
}

struct Data {
    // all accounts must be recoreded in order to output the report
    accounts: HashMap<ClientID, Account>,
    // all transactions must be recoreded in order to process disputes
    txs: HashMap<TxID, Tx>,
    // record all disputes in a hashmap
    disputes: HashMap<TxID, Closed>,
}

impl Data {
    fn new() -> Self {
        Data {
            accounts: HashMap::new(),
            txs: HashMap::new(),
            disputes: HashMap::new(),
        }
    }

    fn credit(&mut self, client_id: ClientID, amount: Decimal) -> TxResult {
        let a = self.accounts.entry(client_id).or_insert_with(Account::new);
        a.credit(amount)
    }

    fn debit(&mut self, client_id: ClientID, amount: Decimal) -> TxResult {
        let a = self.accounts.entry(client_id).or_insert_with(Account::new);
        a.debit(amount)
    }

    fn dispute(&mut self, tx: &Tx) -> TxResult {
        if self.disputes.get(&tx.tx).is_some() {
            return Err(TxError::DuplicateTx);
        }
        let amount = match self.txs.get(&tx.tx) {
            Some(tx) => {
                if tx.type_ != TxType::Deposit {
                    return Err(TxError::TxIsNotCredit);
                }
                tx.amount.unwrap()
            }
            None => return Err(TxError::TxNotFound),
        };
        if let Some(a) = self.accounts.get_mut(&tx.client) {
            a.dispute(amount)?;
        }
        self.disputes.insert(tx.tx, false);
        Ok(())
    }

    fn resolve(&mut self, tx: &Tx) -> TxResult {
        match self.disputes.get(&tx.tx) {
            Some(closed) if *closed => return Err(TxError::DuplicateTx),
            None => return Err(TxError::DisputeNotFound),
            _ => {}
        }
        let amount = self.txs.get(&tx.tx).unwrap().amount.unwrap();
        if let Some(a) = self.accounts.get_mut(&tx.client) {
            a.resolve(amount)?;
        }
        self.disputes.insert(tx.tx, true);
        Ok(())
    }

    fn chargeback(&mut self, tx: &Tx) -> TxResult {
        match self.disputes.get(&tx.tx) {
            Some(closed) if *closed => return Err(TxError::DuplicateTx),
            None => return Err(TxError::DisputeNotFound),
            _ => {}
        }
        let amount = self.txs.get(&tx.tx).unwrap().amount.unwrap();
        if let Some(a) = self.accounts.get_mut(&tx.client) {
            a.chargeback(amount)?;
        }
        self.disputes.insert(tx.tx, true);
        Ok(())
    }

    fn process_transaction(&mut self, tx: &Tx) -> TxResult {
        // record transaction if it is a credit or a debit
        match tx.type_ {
            TxType::Deposit | TxType::Withdrawal => {
                if tx.amount.is_none() {
                    return Err(TxError::BadFormat);
                }
                if self.txs.insert(tx.tx, tx.clone()).is_some() {
                    return Err(TxError::DuplicateTx);
                }
            }
            _ => {}
        };
        // process transaction
        match tx.type_ {
            TxType::Deposit => self.credit(tx.client, tx.amount.unwrap()),
            TxType::Withdrawal => self.debit(tx.client, tx.amount.unwrap()),
            TxType::Dispute => self.dispute(tx),
            TxType::Resolve => self.resolve(tx),
            TxType::Chargeback => self.chargeback(tx),
        }
    }

    fn print_as_csv(&self) {
        println!("client,available,held,total,locked");
        for (id, account) in &self.accounts {
            println!(
                "{},{},{},{},{}",
                id,
                account.available,
                account.held,
                account.available + account.held,
                account.locked
            );
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("Please specify the path of the transactions' file.");
    }

    let mut data = Data::new();

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(args[1].clone())
        .unwrap();

    for result in rdr.deserialize() {
        let tx: Tx = match result {
            Ok(tx) => tx,
            Err(e) => {
                eprintln!("{:?}", e);
                continue;
            }
        };
        if let Err(e) = data.process_transaction(&tx) {
            eprintln!(
                "{:?} (transaction {}) for client {} failed with error {:?}",
                tx.type_, tx.tx, tx.client, e
            );
        }
    }

    data.print_as_csv();
}
