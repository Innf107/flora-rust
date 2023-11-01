use std::marker::PhantomData;

mod syntax;
mod driver;
mod lexer;
mod parser;

fn main() {
    todo!();
}

enum Outcome {
    Ok,
    Err,
    Eff(String, Vec<Value>, Continuation)
}

struct Runtime {
    stack_segments : Vec<StackSegment>
}

struct StackSegment {
    parent : Option<usize>
}

struct Continuation<'runtime> {
    _runtime : PhantomData<&'runtime ()>,
    segment : usize
}

#[cfg(test)]
mod tests {
    use crate::Continuation;

    #[test]
    fn basic() {
        let rt = Runtime::new("let fib(n) = if n < 2 then n else fib(n - 1) + fib(n - 2); perform print(fib(9))");
    
        let mut cont = rt.initial();
        let result = loop {
            match rt.resume(cont) {
                Ok(_) => break "...",
                Eff("print", [Int(89)], next_cont) => {
                    cont = next_cont;
                }
            }
        }
    }

    #[test]
    fn main() {
        let rt = Runtime::new("print(3)");

        struct Cont;
        struct Val;

        enum EffStackNode {
            Handle(String),
            Run(Cont, Vec<Val>)
        }

        let mut eff_stack = vec![rt.initial()];
    }

    #[test]
    fn nondet() {
        let rt = Runtime::new("if perform rand() then 1 else 2");
    
        rt.run_simple(|eff : &str, arguments| {
            
        })

        rt.register_handler("print", |x| {
            println!(x...);
        })

        let mut conts = vec![(rt.initial(), [])];
        let mut results = vec![];

        loop {
            match conts.pop() {
                None => break results.pop();
                Some(Handle(rustcont)) => {
                    results.push(rustcont(results.pop()))
                }
                Some(Handle("random")) => {
                    results.push(combine(results.pop(), results.pop()))
                }
                Some(Run(cont, arguments)) => match rt.resume(cont, arguments) {
                    Ok(x) => conts.push(Result(x)),
                    Eff("random", [], cont) => {
                        conts.push(Handle("random"));
                        conts.push(cont, [true]);

                        // fn resume(cont, arguments, rustcont) {
                        //     conts.push(Handle(rustcont))
                        //     conts.push(cont, arguments)
                        // }

                        // resume(cont, [true] |x results| {
                        //     resume(cont, [false], |y, results| {
                        //         combine(x, y)
                        //     })
                        // })

                        conts.push(Handle)
                        conts.push(Run(cont, [true]));
                        conts.push(Run((cont), [false]));
                      
                    }
                }
            }
        }

        fn handle<R>(cont: Continuation, arguments: &[Value]) {
            match rt.resume(cont, arguments) {
                Eff("print", x, cont) => {
                },
            }
        }

        enum HostCont {
            Continue(Continuation),
            Random1(Continuation, Value),
            Random2(Continuation)
        }
    
        fn handle(cont: HostCont){
            loop {
                match cont {
                    
                }
            }
        }

        fn handle(cont: Continuation, arguments: &[Value]){
            match rt.resume(cont, arguments) {
                Eff("print", x, cont) => {
                    println!(x)
                    handle(cont)
                },
                Eff("random", [], cont) => {
                    let true_case = handle(rt.clone(cont), &[true]);
                    let false_case = handle(cont, &[false]);
                    combine(true_case, false_case);
                }
            }
        }
        handle(rt.initial(), &[]);

        let result = loop {
            match rt.resume(cont) {
                Ok(_) => break "...",
                Eff("print", [Int(89)], next_cont) => {
                    cont = next_cont;
                }
            }
        }
    }
}