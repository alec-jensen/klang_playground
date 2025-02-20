use crate::interpreter::scanner::Value;
use crate::KlangError;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct NativeFn {
    pub name: String,
    pub args: i32,
    pub function: Box<dyn Fn(Vec<Value>) -> Option<Value>>,
}
impl NativeFn {
    pub fn call(&self, args: Vec<Value>) -> Option<Value> {
        (self.function)(args)
    }
}

pub fn create_natives() -> Vec<NativeFn> {
    let mut natives: Vec<NativeFn> = Vec::new();
    natives.extend(math_natives());
    natives.extend(random_natives());
    natives.extend(time_natives());
    natives.extend(vector_natives());
    natives
}
fn math_natives() -> Vec<NativeFn> {
    let mut math_functions: Vec<NativeFn> = Vec::new();
    math_functions.push(NativeFn {
        name: "sin".to_string(),
        args: 1,
        function: Box::new(|args| match args[0] {
            Value::Number(num) => {
                let result = num.sin();
                Some(Value::Number(result))
            }
            _ => {
                error("can only use sin on a number!");
                panic!()
            }
        }),
    });
    math_functions.push(NativeFn {
        name: "cos".to_string(),
        args: 1,
        function: Box::new(|args| match args[0] {
            Value::Number(num) => {
                let result = num.cos();
                Some(Value::Number(result))
            }
            _ => {
                error("can only use cos on a number!");
                panic!()
            }
        }),
    });
    math_functions.push(NativeFn {
        name: "tan".to_string(),
        args: 1,
        function: Box::new(|args| match args[0] {
            Value::Number(num) => {
                let result = num.tan();
                Some(Value::Number(result))
            }
            _ => {
                error("can only use tan on a number!");
                panic!()
            }
        }),
    });
    math_functions.push(NativeFn {
        name: "sqrt".to_string(),
        args: 1,
        function: Box::new(|args| match args[0] {
            Value::Number(num) => {
                let result = num.sqrt();
                Some(Value::Number(result))
            }
            _ => {
                error("can only use sqrt on a number!");
                panic!()
            }
        }),
    });
    math_functions.push(NativeFn {
        name: "pow".to_string(),
        args: 2,
        function: Box::new(|args| match (args[0].clone(), args[1].clone()) {
            (Value::Number(base), Value::Number(exponent)) => {
                let result = base.powf(exponent);
                Some(Value::Number(result))
            }
            _ => {
                error("can only use pow on 2 numbers!");
                panic!()
            }
        }),
    });
    math_functions.push(NativeFn {
        name: "ln".to_string(),
        args: 1,
        function: Box::new(|args| match args[0] {
            Value::Number(num) => {
                let result = num.ln();
                Some(Value::Number(result))
            }
            _ => {
                error("can only use ln on a number!");
                panic!()
            }
        }),
    });
    math_functions.push(NativeFn {
        name: "log".to_string(),
        args: 1,
        function: Box::new(|args| match args[0] {
            Value::Number(num) => {
                let result = num.log10();
                Some(Value::Number(result))
            }
            _ => {
                error("can only use log10 on a number!");
                panic!()
            }
        }),
    });
    math_functions.push(NativeFn {
        name: "round".to_string(),
        args: 1,
        function: Box::new(|args| match args[0] {
            Value::Number(num) => {
                let result = num.round();
                Some(Value::Number(result))
            }
            _ => {
                error("can only use round on a number!");
                panic!()
            }
        }),
    });
    math_functions.push(NativeFn {
        name: "abs".to_string(),
        args: 1,
        function: Box::new(|args| match args[0] {
            Value::Number(x) => Some(Value::Number(x.abs())),
            _ => {
                error("can only use abs on a number!");
                panic!()
            }
        }),
    });
    math_functions.push(NativeFn {
        name: "min".to_string(),
        args: 2,
        function: Box::new(|args| match (args[0].clone(), args[1].clone()) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a.min(b))),
            _ => {
                error("can only use min on 2 numbers!");
                panic!()
            }
        }),
    });
    math_functions.push(NativeFn {
        name: "max".to_string(),
        args: 2,
        function: Box::new(|args| match (args[0].clone(), args[1].clone()) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a.max(b))),
            _ => {
                error("can only use max on 2 numbers!");
                panic!()
            }
        }),
    });
    math_functions.push(NativeFn {
        name: "pi".to_string(),
        args: 0,
        function: Box::new(|_| Some(Value::Number(std::f64::consts::PI))),
    });
    math_functions
}
pub fn random_natives() -> Vec<NativeFn> {
    let mut natives: Vec<NativeFn> = Vec::new();
    natives.push(NativeFn {
        name: "random".to_string(),
        args: 0,
        function: Box::new(|_| {
            let mut rng = rand::thread_rng();
            Some(Value::Number(rng.gen::<f64>()))
        }),
    });
    natives.push(NativeFn {
        name: "range".to_string(),
        args: 2,
        function: Box::new(|args| match (args[0].clone(), args[1].clone()) {
            (Value::Number(min), Value::Number(max)) if min < max => {
                let mut rng = rand::thread_rng();
                let random_value = rng.gen_range(min..max);
                Some(Value::Number(random_value))
            }
            _ => {
                error("can only use random_range on 2 numbers!");
                panic!()
            }
        }),
    });
    natives.push(NativeFn {
        name: "randbool".to_string(),
        args: 0,
        function: Box::new(|_| {
            let mut rng = rand::thread_rng();
            Some(Value::Bool(rng.gen::<bool>()))
        }),
    });
    natives
}

pub fn time_natives() -> Vec<NativeFn> {
    let mut natives: Vec<NativeFn> = Vec::new();
    natives.push(NativeFn {
        name: "time".to_string(),
        args: 0,
        function: Box::new(|_| {
            Some(Value::Number(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs_f64(),
            ))
        }),
    });
    natives.push(NativeFn {
        name: "sleep".to_string(),
        args: 1,
        function: Box::new(|args| match args[0] {
            Value::Number(duration) if duration >= 0.0 => {
                std::thread::sleep(std::time::Duration::from_secs_f64(duration));
                None
            }
            _ => {
                error("can only use sleep on a number!");
                panic!()
            }
        }),
    });
    natives
}
pub fn vector_natives() -> Vec<NativeFn> {
    let mut natives: Vec<NativeFn> = Vec::new();
    natives.push(NativeFn {
        name: "get".to_string(),
        args: 2,
        function: Box::new(
            |mut args| match (args.pop().unwrap(), args.pop().unwrap()) {
                (Value::Number(index), Value::Vec(mut vec)) => Some(vec.remove(index as usize)),
                _ => {
                    error("expected a (vector, number)");
                    panic!()
                }
            },
        ),
    });
    natives.push(NativeFn {
        name: "set".to_string(),
        args: 3,
        function: Box::new(|mut args| {
            if let Value::Number(index) = args.pop().unwrap() {
                let value = args.pop().unwrap();
                if let Value::Vec(mut vec) = args.pop().unwrap() {
                    vec[index as usize] = value;
                    Some(Value::Vec(vec))
                } else {
                    error("expected a (vector, value, index)");
                    panic!()
                }
            } else {
                error("expected a (vector, value, index)");
                panic!()
            }
        }),
    });
    natives.push(NativeFn {
        name: "remove".to_string(),
        args: 2,
        function: Box::new(
            |mut args| match (args.pop().unwrap(), args.pop().unwrap()) {
                (Value::Number(index), Value::Vec(mut vec)) => {
                    vec.remove(index as usize);
                    Some(Value::Vec(vec))
                }
                _ => {
                    error("expected a (vector, number)");
                    panic!()
                }
            },
        ),
    });
    natives.push(NativeFn {
        name: "insert".to_string(),
        args: 3,
        function: Box::new(|mut args| {
            if let Value::Number(index) = args.pop().unwrap() {
                let value = args.pop().unwrap();
                if let Value::Vec(mut vec) = args.pop().unwrap() {
                    vec.insert(index as usize, value);
                    Some(Value::Vec(vec))
                } else {
                    error("expected a (vector, value, index)");
                    panic!()
                }
            } else {
                error("expected a (vector, value, index)");
                panic!()
            }
        }),
    });
    natives.push(NativeFn {
        name: "len".to_string(),
        args: 1,
        function: Box::new(|mut args| {
            if let Value::Vec(vec) = args.pop().unwrap() {
                Some(Value::Number(vec.len() as f64))
            } else {
                error("expected a (vector)");
                panic!()
            }
        }),
    });

    natives
}

fn error(msg: &str) {
    KlangError::error(KlangError::RuntimeError, msg, 0);
}
