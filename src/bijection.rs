use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Fn;

struct TwoWayMap<A, B>
where A: Eq + Hash, B: Eq + Hash {
    forward: HashMap<A, B>,
    backward: HashMap<B, A>
}

impl<A: Eq + Hash + Clone, B: Eq + Hash + Clone> TwoWayMap<A, B> {
    fn empty() -> TwoWayMap<A, B> {
        TwoWayMap { forward: HashMap::new(), backward: HashMap::new() }
    }

    fn map(&mut self, input: A, output: B) {
        self.forward.insert(input.clone(), output.clone());
        self.backward.insert(output, input);
    }

    fn input(&self, input: &A) -> Option<&B> {
        self.forward.get(input)
    }
}

pub struct Bijection<A: Eq + Hash + Clone, B: Eq + Hash + Clone> {
    function: Box<Fn(A) -> B + Send + 'static>,
    memo_table: RefCell<TwoWayMap<A, B>>
}

impl<A: Eq + Hash + Clone, B: Eq + Hash + Clone> Bijection<A, B> {
    pub fn new<F: Fn(A) -> B + Send + 'static>(f: F) -> Bijection<A, B> {
        Bijection {
            function: Box::new(f),
            memo_table: RefCell::new(TwoWayMap::empty())
        }
    }

    fn run(&self, arg: A) -> B {
        let &Bijection {
            ref function,
            ref memo_table,
        } = self;

        let mut memo_table = memo_table.borrow_mut();

        match memo_table.input(&arg).map(|b| b.clone()) {
            Some(b) => b,
            None => {
                let result = (*self.function)(arg.clone());
                memo_table.map(arg.clone(), result.clone());
                result
            }
        }
    }
}

impl<A: Eq + Hash + Clone, B: Eq + Hash + Clone> FnOnce<(A,)> for Bijection<A, B> {
    type Output = B;

    extern "rust-call" fn call_once(self, args: (A,)) -> B {
        self.run(args.0)
    }
}

impl<A: Eq + Hash + Clone, B: Eq + Hash + Clone> FnMut<(A,)> for Bijection<A, B> {
    extern "rust-call" fn call_mut(&mut self, args: (A,)) -> B {
        self.run(args.0)
    }
}

impl<A: Eq + Hash + Clone, B: Eq + Hash + Clone> Fn<(A,)> for Bijection<A, B> {
    extern "rust-call" fn call(&self, args: (A,)) -> B {
        self.run(args.0)
    }
}

#[test]
fn test_only_computes_once() {
    let mut i = 0;
    let map_to_n_plus_1 = Bijection::new(|x| {
        i += 1;
        x + 1
    });
    map_to_n_plus_1(10);
    map_to_n_plus_1(10);
    assert_eq!(i, 1);
}
