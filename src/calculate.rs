use std::collections::HashMap;

enum Operation {
	Add(f64),
	Subtract(f64),
	Multiply(f64),
	Divide(f64),
	Raise(f64),
	Root(f64, Option<f64>)
}

use self::Operation::*;

pub fn parse_string(string: String, evironment: Option<HashMap<String, f64>>) -> Result<f64, &'static str> {
	let digits: &[_] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', ','];
	let operators: &[_] = &['+', '-', '*', '×', '/', '÷', '^', ];
	let mut operations: Vec<Operation> = Vec::new();

	// Remove whitespace
	let mut string = string.replace(char::is_whitespace, "");

	// Finds number by draining string unil location of next operator (len)
	let get_number = |s: &mut String| -> f64 {
		let len = s.find(operators).unwrap_or(s.len());
		s.drain(..len).collect::<String>().parse::<f64>().expect("Invalid input")
	};

	// If first character is number, assume + operation
	if string.find(digits).unwrap_or(string.len()) == 0 {
		operations.push(Add(get_number(&mut string)));
	}

	while string.len() > 0 {
		let c = string.remove(0);
		match c {
			'+' => operations.push(Add(get_number(&mut string))),
			'-' => operations.push(Subtract(get_number(&mut string))),
			'^' => operations.push(Raise(get_number(&mut string))),
			'/' | '÷' => operations.push(Divide(get_number(&mut string))),
			'*' | '×' | '·' | '⋅' => operations.push(Multiply(get_number(&mut string))),
			_ => return Err("Syntax error"),
		}
		if string.len() == 0 { break; }
	}

	let mut result = 0f64;

	// Resolve powers
    let mut index = 0;
    while index < operations.len() {
        // Match power at index and assign value
		if let &Raise(val) = &operations[index] {
            // Match previous operation
            // TODO: Can the enum be '_'?
			match operations[index-1] {
				_(ref mut prev_val: f64) => *prev_val = prev_val.powf(val),
				/*Add(ref mut prev_val) => *prev_val = prev_val.powf(val),
				Subtract(ref mut prev_val) => *prev_val = prev_val.powf(val),
				Multiply(ref mut prev_val) => *prev_val = prev_val.powf(val),
				Divide(ref mut prev_val) => *prev_val = prev_val.powf(val),
				Raise(ref mut prev_val) => *prev_val = prev_val.powf(val),*/
				_ => return Err("Operation not implemented yet"),
			}
			operations.remove(index);
		}
        index += 1;
	}
	// Resolve divisions
    let mut index = 0;
    while index < operations.len() {
        // Match division at index and assign value
        if let &Divide(val) = &operations[index] {
            match operations[index-1] {
                Add(ref mut prev_val) => *prev_val /= val,
                Subtract(ref mut prev_val) => *prev_val /= val,
                Multiply(ref mut prev_val) => *prev_val /= val,
                Divide(ref mut prev_val) => *prev_val /= val,
                Raise(ref mut prev_val) => *prev_val /= val,
				_ => return Err("Operation not implemented yet"),
            }
			operations.remove(index);
        }
        index += 1;
    }
	// Resolve products
    let mut index = 0;
    while index < operations.len() {
        // Match multiplication at index and assign value
        if let &Multiply(val) = &operations[index] {
            match operations[index-1] {
                Add(ref mut prev_val) => *prev_val *= val,
                Subtract(ref mut prev_val) => *prev_val *= val,
                Multiply(ref mut prev_val) => *prev_val *= val,
                Divide(ref mut prev_val) => *prev_val *= val,
                Raise(ref mut prev_val) => *prev_val *= val,
				_ => return Err("Operation not implemented yet"),
            }
			operations.remove(index);
        }
        index += 1;
    }
	// Parse sums and differences
	for op in operations {
		match op {
			Add(val) => result = result + val,
			Subtract(val) => result = result - val,
			_ => return Err("Syntax error: unresolved operations"),
		}
	}

	Ok(result)
}
