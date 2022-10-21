use std::io::{stdin,stdout,Write};

fn read(input:&mut String){        // define input and out and flush the previous i/p
    stdout().flush().expect("failed to remove");
    stdin().read_line(input).expect("failed to read");
}

fn main(){
    loop{
        let mut select_function = String::new();

        println!("Select 1 to Print Stack or 2 for Calculator");
        read(&mut select_function);

        let select_function: char =select_function.trim().parse().unwrap();

        let select_functions= String:: from("12");

        if !select_functions.contains(select_function){
            println!("Unknown operators");
            continue;
        }

        let result= match select_function{
            '1'=> println!("print stack"),
            '2'=> calculator(),
            _=> panic!("Error in Operator")

        };
    }
}

fn calculator(){
    loop{
        let mut num1 = String::new();         //take all i/p as string 
        let mut num2 =String::new();
        let mut operator = String::new();
    
        println!("Enter First Digit:"); // read  i/p to as string 
        read(&mut num1);
    
        println!("Enter Second Digit:");
        read(&mut num2);
        
        println!("Enter the operator you would like [+,-,*,/,%,and=a ,or =o]:");
        read(&mut operator);
    
        let num1: f32 =num1.trim().parse().unwrap();// conver string to integer 
        let num2: f32 =num2.trim().parse().unwrap();
        let operator: char =operator.trim().parse().unwrap();
    
        let operators = String:: from("+-*/%ao");
    
        if !operators.contains(operator){
            println!("Unknown operators");
            continue;
        }
    
        let result= match operator{
            '+'=> num1 + num2,
            '-'=> num1 - num2,
            '*'=> num1 * num2,
            '/'=> num1 / num2,
            '%'=> num1 % num2,
            'a'=> num1 * num2,
            'o'=> num1 + num2,
            _=> panic!("Error in Operator")
    
        };
    
        println!("The result is: {} {} {} ={}",num1,operator,num2,result);
        println!("------------End-------------- ");
      }
}

