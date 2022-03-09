# csv_bank

# Debug info for some test cases

## transctions.csv
```
client,available,held,total,
1,0.0000,0.0000,0.0000,false
2,2.0000,0.0000,2.0000,false
Bank {
    accounts: {
        1: Client {
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        },
        2: Client {
            available: 2.0,
            held: 0.0,
            total: 2.0,
            locked: false,
        },
    },
    valid_deposits: {
        1: Tx {
            ty: Deposit,
            cid: 1,
            tid: 1,
            amount: Some(
                1.0,
            ),
        },
        3: Tx {
            ty: Deposit,
            cid: 1,
            tid: 3,
            amount: Some(
                3.0,
            ),
        },
        2: Tx {
            ty: Deposit,
            cid: 2,
            tid: 2,
            amount: Some(
                2.0,
            ),
        },
    },
    valid_withdrawals: {
        4: Tx {
            ty: Withdrawal,
            cid: 1,
            tid: 4,
            amount: Some(
                4.0,
            ),
        },
    },
    valid_disputes: {},
    invalid_txs: [
        (
            Tx {
                ty: Withdrawal,
                cid: 2,
                tid: 5,
                amount: Some(
                    5.0,
                ),
            },
            FundNotAvailable,
        ),
    ],
}
```

## dispute.csv
```
client,available,held,total,
2,2.0000,0.0000,2.0000,false
1,-1.0000,1.0000,0.0000,false
Bank {
    accounts: {
        2: Client {
            available: 2.0,
            held: 0.0,
            total: 2.0,
            locked: false,
        },
        1: Client {
            available: -1.0,
            held: 1.0,
            total: 0.0,
            locked: false,
        },
    },
    valid_deposits: {
        3: Tx {
            ty: Deposit,
            cid: 1,
            tid: 3,
            amount: Some(
                3.0,
            ),
        },
        1: Tx {
            ty: Deposit,
            cid: 1,
            tid: 1,
            amount: Some(
                1.0,
            ),
        },
        2: Tx {
            ty: Deposit,
            cid: 2,
            tid: 2,
            amount: Some(
                2.0,
            ),
        },
    },
    valid_withdrawals: {
        4: Tx {
            ty: Withdrawal,
            cid: 1,
            tid: 4,
            amount: Some(
                4.0,
            ),
        },
    },
    valid_disputes: {
        1: Pending,
    },
    invalid_txs: [
        (
            Tx {
                ty: Withdrawal,
                cid: 2,
                tid: 5,
                amount: Some(
                    5.0,
                ),
            },
            FundNotAvailable,
        ),
    ],
}
```

## resolve.csv
```
client,available,held,total,
1,0.0000,0.0000,0.0000,false
2,2.0000,0.0000,2.0000,false
Bank {
    accounts: {
        1: Client {
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        },
        2: Client {
            available: 2.0,
            held: 0.0,
            total: 2.0,
            locked: false,
        },
    },
    valid_deposits: {
        2: Tx {
            ty: Deposit,
            cid: 2,
            tid: 2,
            amount: Some(
                2.0,
            ),
        },
        3: Tx {
            ty: Deposit,
            cid: 1,
            tid: 3,
            amount: Some(
                3.0,
            ),
        },
        1: Tx {
            ty: Deposit,
            cid: 1,
            tid: 1,
            amount: Some(
                1.0,
            ),
        },
    },
    valid_withdrawals: {
        4: Tx {
            ty: Withdrawal,
            cid: 1,
            tid: 4,
            amount: Some(
                4.0,
            ),
        },
    },
    valid_disputes: {
        1: Resolved,
    },
    invalid_txs: [
        (
            Tx {
                ty: Withdrawal,
                cid: 2,
                tid: 5,
                amount: Some(
                    5.0,
                ),
            },
            FundNotAvailable,
        ),
    ],
}
```

## chargeback.csv
```
client,available,held,total,
1,-1.0000,0.0000,-1.0000,true
2,2.0000,0.0000,2.0000,false
Bank {
    accounts: {
        1: Client {
            available: -1.0,
            held: 0.0,
            total: -1.0,
            locked: true,
        },
        2: Client {
            available: 2.0,
            held: 0.0,
            total: 2.0,
            locked: false,
        },
    },
    valid_deposits: {
        1: Tx {
            ty: Deposit,
            cid: 1,
            tid: 1,
            amount: Some(
                1.0,
            ),
        },
        3: Tx {
            ty: Deposit,
            cid: 1,
            tid: 3,
            amount: Some(
                3.0,
            ),
        },
        2: Tx {
            ty: Deposit,
            cid: 2,
            tid: 2,
            amount: Some(
                2.0,
            ),
        },
    },
    valid_withdrawals: {
        4: Tx {
            ty: Withdrawal,
            cid: 1,
            tid: 4,
            amount: Some(
                4.0,
            ),
        },
    },
    valid_disputes: {
        1: Chargeback,
    },
    invalid_txs: [
        (
            Tx {
                ty: Withdrawal,
                cid: 2,
                tid: 5,
                amount: Some(
                    5.0,
                ),
            },
            FundNotAvailable,
        ),
    ],
}
```