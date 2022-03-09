use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::slice::Iter;

#[derive(Debug, Clone, Copy)]
enum TxType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Clone, Copy)]
struct Tx {
    ty: TxType,
    cid: u16,
    tid: u32,
    amount: Option<f64>,
}

impl Tx {
    fn new(words: &[&str]) -> Self {
        assert!(words.len() >= 3, "transaction data invalid");
        let cid = words[1].trim().parse().expect("client id is invalid");
        let tid = words[2].trim().parse().expect("tx id is invalid");
        match words[0] {
            "deposit" => {
                assert!(words.len() == 4, "transaction data invalid");
                Tx {
                    ty: TxType::Deposit,
                    cid,
                    tid,
                    amount: Some(words[2].trim().parse::<f64>().expect("tx id is invalid")),
                }
            }
            "withdrawal" => {
                assert!(words.len() == 4, "transaction data invalid");
                Tx {
                    ty: TxType::Withdrawal,
                    cid,
                    tid,
                    amount: Some(words[2].trim().parse::<f64>().expect("tx id is invalid")),
                }
            }
            "dispute" => {
                assert!(words.len() == 3, "transaction data invalid");
                Tx {
                    ty: TxType::Dispute,
                    cid,
                    tid,
                    amount: None,
                }
            }
            "resolve" => {
                assert!(words.len() == 3, "transaction data invalid");
                Tx {
                    ty: TxType::Resolve,
                    cid,
                    tid,
                    amount: None,
                }
            }
            "chargeback" => {
                assert!(words.len() == 3, "transaction data invalid");
                Tx {
                    ty: TxType::Chargeback,
                    cid,
                    tid,
                    amount: None,
                }
            }
            _ => {
                panic!("type is invalid");
            }
        }
    }
}

#[derive(Debug, Default)]
struct Client {
    available: f64,
    held: f64,
    total: f64,
    locked: bool,
}

impl Client {
    fn deposit(&mut self, amount: f64) {
        self.available += amount;
        self.total += amount;
    }
    fn withdrawal(&mut self, amount: f64) -> bool {
        if amount <= self.available {
            self.available -= amount;
            self.total -= amount;
            true
        } else {
            false
        }
    }
    fn dispute(&mut self, amount: f64) {
        self.available -= amount;
        self.held += amount;
    }
    fn resolve(&mut self, amount: f64) {
        self.available += amount;
        self.held -= amount;
    }
    fn chargeback(&mut self, amount: f64) {
        self.held -= amount;
        self.total -= amount;
        self.locked = true;
    }
}

#[derive(Debug)]
enum TxError {
    Unknown,
    TransactionIdDuplicated,
    AccountLocked,
    FundNotAvailable,
    ValidDepositNotFound,
    ClientIdNotMatching,
    ValidDesputeNotFound,
    DisputeStatusNotPending,
}

impl Default for TxError {
    fn default() -> Self {
        TxError::Unknown
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum DisputeStatus {
    Pending,
    Resolved,
    Chargeback,
}

impl Default for DisputeStatus {
    fn default() -> Self {
        DisputeStatus::Pending
    }
}

#[derive(Debug, Default)]
struct Bank {
    accounts: HashMap<u16, Client>,
    valid_deposits: HashMap<u32, Tx>,
    valid_withdrawals: HashMap<u32, Tx>,
    valid_disputes: HashMap<u32, DisputeStatus>,
    invalid_txs: Vec<(Tx, TxError)>,
}

impl Bank {
    fn process_txs(&mut self, mut it: Iter<Tx>) {
        while let Some(&tx) = it.next() {
            self.process_tx(tx);
        }
    }

    fn process_tx(&mut self, tx: Tx) {
        use TxType::*;

        let client = self.accounts.entry(tx.cid).or_default();
        match tx.ty {
            Deposit => {
                let amount = tx.amount.expect("amount is missing");
                if client.locked {
                    self.invalid_txs.push((tx, TxError::AccountLocked));
                    return;
                }
                if self.valid_deposits.contains_key(&tx.tid) {
                    self.invalid_txs
                        .push((tx, TxError::TransactionIdDuplicated));
                    return;
                }
                client.deposit(amount);
                self.valid_deposits.insert(tx.tid, tx);
            }
            Withdrawal => {
                let amount = tx.amount.expect("amount is missing");
                if client.locked {
                    self.invalid_txs.push((tx, TxError::AccountLocked));
                    return;
                }
                if self.valid_withdrawals.contains_key(&tx.tid) {
                    self.invalid_txs
                        .push((tx, TxError::TransactionIdDuplicated));
                    return;
                }
                if !client.withdrawal(amount) {
                    self.invalid_txs.push((tx, TxError::FundNotAvailable));
                    return;
                }
                self.valid_withdrawals.insert(tx.tid, tx);
            }
            Dispute => {
                if client.locked {
                    self.invalid_txs.push((tx, TxError::AccountLocked));
                    return;
                }
                if self.valid_disputes.contains_key(&tx.tid) {
                    self.invalid_txs
                        .push((tx, TxError::TransactionIdDuplicated));
                    return;
                }
                if !self.valid_deposits.contains_key(&tx.tid) {
                    self.invalid_txs.push((tx, TxError::ValidDepositNotFound));
                    return;
                }
                let deposit = self
                    .valid_deposits
                    .get(&tx.tid)
                    .expect("no valid deposit found");
                if deposit.cid != tx.cid {
                    self.invalid_txs.push((tx, TxError::ClientIdNotMatching));
                    return;
                }
                client.dispute(deposit.amount.expect("amount is missing"));
                self.valid_disputes.insert(tx.tid, DisputeStatus::Pending);
            }
            Resolve => {
                if client.locked {
                    self.invalid_txs.push((tx, TxError::AccountLocked));
                    return;
                }
                if !self.valid_disputes.contains_key(&tx.tid) {
                    self.invalid_txs.push((tx, TxError::ValidDesputeNotFound));
                    return;
                }
                if !self.valid_deposits.contains_key(&tx.tid) {
                    self.invalid_txs.push((tx, TxError::ValidDepositNotFound));
                    return;
                }
                let deposit = self
                    .valid_deposits
                    .get(&tx.tid)
                    .expect("no valid deposit found");
                if deposit.cid != tx.cid {
                    self.invalid_txs.push((tx, TxError::ClientIdNotMatching));
                    return;
                }
                let dispute = self
                    .valid_disputes
                    .get(&tx.tid)
                    .expect("no valid dispute found");
                if dispute != &DisputeStatus::Pending {
                    self.invalid_txs
                        .push((tx, TxError::DisputeStatusNotPending));
                    return;
                }
                client.resolve(deposit.amount.expect("amount is missing"));
                self.valid_disputes.insert(tx.tid, DisputeStatus::Resolved);
            }
            Chargeback => {
                if client.locked {
                    self.invalid_txs.push((tx, TxError::AccountLocked));
                    return;
                }
                if !self.valid_disputes.contains_key(&tx.tid) {
                    self.invalid_txs.push((tx, TxError::ValidDesputeNotFound));
                    return;
                }
                if !self.valid_deposits.contains_key(&tx.tid) {
                    self.invalid_txs.push((tx, TxError::ValidDepositNotFound));
                    return;
                }
                let deposit = self
                    .valid_deposits
                    .get(&tx.tid)
                    .expect("no valid deposit found");
                if deposit.cid != tx.cid {
                    self.invalid_txs.push((tx, TxError::ClientIdNotMatching));
                    return;
                }
                let dispute = self
                    .valid_disputes
                    .get(&tx.tid)
                    .expect("no valid dispute found");
                if dispute != &DisputeStatus::Pending {
                    self.invalid_txs
                        .push((tx, TxError::DisputeStatusNotPending));
                    return;
                }
                client.chargeback(deposit.amount.expect("amount is missing"));
                self.valid_disputes
                    .insert(tx.tid, DisputeStatus::Chargeback);
            }
        }
    }

    fn print(&self) {
        println!("client,available,held,total,");
        for (id, client) in self.accounts.iter() {
            println!(
                "{},{:.4},{:.4},{:.4},{}",
                id, client.available, client.held, client.total, client.locked
            );
        }
        // println!("{:#?}", self);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let txs = cli(args);
    let mut bank = Bank::default();
    bank.process_txs(txs.iter());
    bank.print();
}

fn cli(args: Vec<String>) -> Vec<Tx> {
    let filename = &args[1];
    let file = File::open(filename).expect("CSV can't be opened");
    let reader = BufReader::new(file);
    let lines = reader.lines().skip(1).map(|l| l.expect("read line error"));
    let mut txs: Vec<Tx> = vec![];
    for line in lines {
        let words: Vec<&str> = line.split(',').collect();
        let tx = Tx::new(&words);
        txs.push(tx);
    }
    txs
}

#[test]
fn test() {
    cli(vec!["".to_string(), "transactions.csv".to_string()]);
    cli(vec!["".to_string(), "dispute.csv".to_string()]);
    cli(vec!["".to_string(), "resolve.csv".to_string()]);
    cli(vec!["".to_string(), "chargeback.csv".to_string()]);
}
