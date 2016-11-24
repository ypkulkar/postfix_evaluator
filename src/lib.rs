pub enum Operator {
    // `+`
    Add, 
    // `-`
    Sub,
    // `*`
    Mul,
}

pub enum Token {
    Operator(Operator),
    Operand(isize),
}

pub fn eval(tokens: &[Token]) -> Option<isize> {
    
    
    
    let mut stack = vec![];
    
    for token in tokens{
       match token{
            &Token::Operand(token) => stack.push(token),
           	
           	&Token::Operator(ref x) => {

           		match x{
           			&Operator::Add => {
           				if stack.len() < 2{
           					return None;
           				}
           				else{
	           				let n = stack.pop().unwrap();
	           				let m = stack.pop().unwrap();
	           				let val = m+n;
	           				stack.push(val);
	           			}

           			}
           			&Operator::Sub => {
           				if stack.len() < 2{
           					return None;
           				}
           				else{
	           				let n = stack.pop().unwrap();
	           				let m = stack.pop().unwrap();
	           				let val = m-n;
	           				stack.push(val);
	           			}
           			}
           			&Operator::Mul => {
           				if stack.len() < 2{
           					return None;
           				}
           				else{
	           				let n = stack.pop().unwrap();
	           				let m = stack.pop().unwrap();
	           				let val = m*n;
	           				stack.push(val);
	           			}
           			}
           		}

           	}
       
       
        }
    }

    if stack.len() == 1{
    	return Some(stack[0]);
    }
    else{
    	return None;
    }
    
    
    
    
}


     
        
#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn it_works() {
    	assert_eq!(Some(5), eval(&[Token::Operand(2),Token::Operand(3),Token::Operator(Operator::Add)]))
    }
}