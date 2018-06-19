// util

use tx::tx::{Transaction};
use tx::outputs::{TXOutput};
use tx::inputs::{TXInput};
use blockchain::blockchain::{Blockchain};

pub fn new_coinbase_tx(to: String, data: String) -> Transaction {
    println!("{:?}", data);
    let subsidy = 1;
    let txin = TXInput {
        txid: vec![],
        vout_idx: -1,
        signature: vec![],
        pub_key: vec![]
    };
    let txout = TXOutput{
        value: subsidy,
        pubkey_hash: to.into_bytes(),
    };
    let mut tx = Transaction {
        id: vec![],
        vin: vec![txin],
        vout: vec![txout]
    };
    tx = tx.set_id();

    return tx;
}

pub fn new_utxo_transaction(_to: String, _from: String, _amount: i32, _bc: Blockchain) -> Option<Transaction> {
    let mut _inputs: Vec<TXInput> = vec![];
    let mut _outputs: Vec<TXOutput> = vec![];
    let (_acc, _valid_outputs) = _bc.find_spendable_outputs(_from.to_owned(), _amount);

    for (_txid, _outs) in _valid_outputs.clone().iter() {
        let _tx_id = _txid.to_owned().into_bytes();
        
        for out in _outs {
            let _input = TXInput{
                txid: _tx_id.to_owned(),
                vout_idx: out.to_owned(),
                signature: _from.to_owned().into_bytes(),
                pub_key: vec![]
            };
            _inputs.append(&mut vec![_input]);
        }
    }

    if _acc < _amount { return None; }

    _outputs.append(&mut vec![TXOutput{
        value: _amount,
        pubkey_hash: _to.into_bytes()
    }]);
    
    
    _outputs.append(&mut vec![TXOutput{
        value:  -_amount,
        pubkey_hash: _from.into_bytes()
    }]);        
    

    let mut _tx = Transaction{ id: vec![], vin: _inputs, vout: _outputs };
    _tx = _tx.set_id();
    //_bc.sign_transaction(_tx.to_owned(), wallet.priv_key);
    
    return Some(_tx);
}
