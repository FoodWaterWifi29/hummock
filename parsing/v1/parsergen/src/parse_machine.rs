mod discard;

use std::marker::PhantomData;
use discard::HasDiscardFunc;
use crate::list::*;
// comment_text = +(whitespace | letter);
//
// comment_text = (whitespace | letter) *(whitespace | letter);
//
// comment_text = (whitespace | letter) ?comment_text_star1;
// comment_text_star1 = (whitespace | letter) ?comment_text_star1;
//
// comment_text = first *(whitespace | letter) rest;
//
// comment_text = first comment_text_star1 rest;
// comment_text_star1 = ?((whitespace | letter) comment_text_star1);

pub trait ParseRule<SymbolType, RuleType>
    where SymbolType: Copy, RuleType: Copy
{
    fn root() -> RuleType;
    fn execute(&self, symbol: SymbolType, stack: List<SymbolOrRule<SymbolType, RuleType>>) -> List<SymbolOrRule<SymbolType, RuleType>>;
}

#[derive(Copy, Clone)]
pub enum SymbolOrRule<SymbolType, RuleType>
    where SymbolType: Copy, RuleType: Copy
{
    Symbol(SymbolType),
    Rule(RuleType)
}

pub struct ParseMachine<SymbolType, RuleType>
    where SymbolType: Copy, RuleType: Copy
{
    // The branches currently being considered by the parse machine.
    // Starts with a length of 1, but grows whenever the parser encounters ambiguous parsings.
    // Multiple branches indicate that the parse machine is in a disambiguating state. Over time,
    // these ambiguities will be resolved as more symbols are passed and branches hit dead ends.
    branches: Vec<ParseBranch<SymbolType, RuleType>>,

    // Phantom data is neccessary since Vec is invariant on SymbolType and RuleType.
    phantom1: PhantomData<SymbolType>,
    phantom2: PhantomData<RuleType>
}

pub enum RejectReason
{
    // The parse machine has hit a parse error, i.e. all branches are dead.
    Error,

    // The parse machine is in an ambiguous terminal state, i.e. multiple branches are alive. Fix your language to not do this.
    Ambiguous,

    // The parse machine was given input when already in a terminal state. Fix your program to not do this.
    AlreadyTerminal
}

pub enum ProcessResult
{
    // The parse machine is awaiting more input.
    Awaiting,

    // The parse machine has accepted, i.e. finished parsing its input and reached a terminal state.
    Accepted
}

pub enum ReadResult<SymbolType, RuleType>
    where SymbolType: Copy, RuleType: Copy
{   
    // The parse machine has rejected the input for one reason or another. It is now in a terminal state.
    Rejected{reason: RejectReason},
   
    // The parse machine has processed the input successfully. Always check result and symbols. Symbols may be empty if the parser is disambiguating.
    Processed{result: ProcessResult, symbols: Vec<SymbolOrRule<SymbolType, RuleType>>}
}

impl<SymbolType, RuleType> ParseMachine<SymbolType, RuleType>
    where SymbolType: Copy, SymbolType: 'static, RuleType: Copy, RuleType: 'static, RuleType: ParseRule<SymbolType, RuleType>
{
    pub fn new() {
        // Create a new parse machine. Its branches will contain one branch containing the root rule on the stack.
        ParseMachine{
            branches: Vec::from([ParseBranch::<SymbolType, RuleType>::new()]),
            phantom1: PhantomData,
            phantom2: PhantomData
        };
    }

    pub fn read(&mut self, symbol: SymbolType) -> ReadResult<SymbolType, RuleType>
    {        
        let mut num_accepted_branches: usize = 0;

        for branch in &mut self.branches {
            let (new_stack, result) = match std::mem::take(&mut branch.stack).state() {
                NonEmptyList(head, tail) => {
                    (
                        // new_stack
                        match head {
                            SymbolOrRule::Symbol(_) => None,
                            SymbolOrRule::Rule(rule) => Some(rule.execute(symbol, tail))
                        },

                        // result
                        Some(head.clone())
                    )
                },

                // If the branch has accepted, its stack will be empty.
                EmptyList => (None, None)
            };

            if let Some(new_stack) = new_stack {
                branch.stack = new_stack;
            }

            if let Some(result) = result {
                branch.parsed = List::cons(result, std::mem::take(&mut branch.parsed));
            }

            if branch.is_accepted() {
                num_accepted_branches += 1;
            }
        };

        // Prune dead branches. Accepted branches won't be pruned because they are still alive.
        self.branches.discard(|branch| !branch.alive);

        match (self.branches.len(), num_accepted_branches) {

            // Will return Processed when there is only one branch alive.
            (1, num_accepted_branches) => if let Some(branch) = self.branches.get_mut(0) {
                ReadResult::Processed {

                    // Return Accepted if there is one accepted branch; otherwise return Awaiting.
                    result: if num_accepted_branches == 1 { ProcessResult::Accepted } else { ProcessResult::Awaiting },

                    // Empty the branch's parsed symbols into the result enum. It will be empty upon the next read.
                    symbols: std::mem::take(&mut branch.parsed).reverse_into_vec()
                }
            } else { panic!() },

            // Will reject due to parse error if all branches are dead.
            (0, 0) => ReadResult::Rejected{ reason: RejectReason::Error },

            // Will reject due to parse ambiguity if the number of alive branches equals the number of accepted branches.
            (n, m) if n == m => ReadResult::Rejected { reason: RejectReason::Ambiguous },

            // There are multiple branches, so the parse machine cannot make a decision. It must await input for disambiguating.
            _ => ReadResult::Processed{ result: ProcessResult::Awaiting, symbols: vec![] }
        }
    }
}

struct ParseBranch<SymbolType, RuleType>
    where SymbolType: Copy, RuleType: Copy
{
    stack: List<SymbolOrRule<SymbolType, RuleType>>,
    parsed: List<SymbolOrRule<SymbolType, RuleType>>,
    alive: bool
}

impl<SymbolType, RuleType> ParseBranch<SymbolType, RuleType>
    where SymbolType: Copy, RuleType: Copy, RuleType: ParseRule<SymbolType, RuleType>
{
    fn new() -> Self {
        Self{
            stack: List::cons(SymbolOrRule::Rule(RuleType::root()), List::EMPTY),
            parsed: List::EMPTY,
            alive: true
        }
    }

    fn clone(&self) -> Self {
        Self{
            stack: self.stack.clone(),
            parsed: self.parsed.clone(),
            alive: self.alive
        }
    }

    fn is_accepted(&self) -> bool {
        self.alive && self.stack.is_empty()
    }
}
