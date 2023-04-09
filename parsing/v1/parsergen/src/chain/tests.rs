use crate::chain::*;

fn sum_chain_internal(chain: Chain<i32>, acc: i32) -> i32 {
    match chain.state() {
        NonEmptyChain(head, tail) => sum_chain_internal(tail, head + acc),
        EmptyChain => acc
    }
}

fn sum_chain(list: Chain<i32>) -> i32 {
    sum_chain_internal(list, 0)
}

#[test]
fn test_sum_chain() {
    let chain = Chain::cons(1, Chain::cons(2, Chain::cons(4, Chain::EMPTY)));
    let chain2 = Chain::concat(chain.clone(), chain.clone());
    assert_eq!(sum_chain(chain), 7);
    assert_eq!(sum_chain(chain2), 14);
}

#[test]
fn test_sum_concat_chain() {
    let chain1 = Chain::cons(1, Chain::cons(2, Chain::cons(4, Chain::EMPTY)));
    let chain2 = Chain::cons(3, Chain::cons(8, Chain::cons(2, Chain::EMPTY)));
    let chain3 = Chain::concat(chain1, chain2);
    assert_eq!(sum_chain(chain3), 20);
}

#[test]
fn test_sum_slice_chain() {
    let chain1 = Chain::cons(1, Chain::cons(2, Chain::cons(4, Chain::EMPTY)));
    let chain2 = Chain::cons(3, Chain::cons(8, Chain::cons(2, Chain::EMPTY)));
    let chain3 = Chain::concat(chain1, chain2);
    if let ChainState::NonEmptyChain(_, tail) = chain3.state() {
        assert_eq!(sum_chain(Chain::slice(tail, 4)), 17);
    } else {
        panic!();
    }
}