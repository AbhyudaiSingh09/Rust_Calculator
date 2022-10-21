use std::io::{stdin,stdout,Write};

fn read(input:&mut String){        // define input and out and flush the previous i/p
    stdout().flush().expect("failed to remove");
    stdin().read_line(input).expect("failed to read");
}



fn main(){
    loop{
        let mut select_function = String::new();

        println!("Select 1 = Print Stack, 2 = Airthemetic  Calculator,3 = Bitwise Calculator and  4 = Negate_calculator");
        read(&mut select_function);

        let select_function: char =select_function.trim().parse().unwrap();

        let select_functions= String:: from("1234");

        if !select_functions.contains(select_function){
            println!("Unknown operators");
            continue;
        }

        let result= match select_function{
            '1'=> println!("print stack"),
            '2'=> airthmetic_calculator(),
            '3'=> bitwise_calculator(),
            '4'=> negate_calculator(),
            _=> panic!("Error in Operator")

        };
    }
}

fn airthmetic_calculator(){

    println!("airthmetic Calculator");
 
    let mut num1 = String::new();         //take all i/p as string 
    let mut num2 =String::new();
    let mut operator = String::new();
    
    println!("Enter First Digit:"); // read  i/p to as string 
    read(&mut num1);
    
    println!("Enter Second Digit:");
    read(&mut num2);
        
    println!("Enter the operator you would like [+,-,*,/,%]:");
    read(&mut operator);
    
    let num1: f32 =num1.trim().parse().unwrap();// conver string to integer 
    let num2: f32 =num2.trim().parse().unwrap();
    let operator: char =operator.trim().parse().unwrap();
    
    let operators = String:: from("+-*/%ao");
    
    if !operators.contains(operator){
        println!("Unknown operators");
    }
    
    let result= match operator{
        '+'=> num1 + num2,
        '-'=> num1 - num2,
        '*'=> num1 * num2,
        '/'=> num1 / num2,
        '%'=> num1 % num2,
        _=> panic!("Error in Operator")
    
    };
    
    println!("The result is: {} {} {} ={}",num1,operator,num2,result);
    println!("------------End-------------- ");
}

fn bitwise_calculator(){
    println!("Bitwise Calculator");
    let mut num1 = String::new();         //take all i/p as string 
    let mut num2 =String::new();
    let mut operator = String::new();

    println!("Enter First Digit:"); // read  i/p to as string 
    read(&mut num1);

    println!("Enter Second Digit:");
    read(&mut num2);
    
    println!("Enter the operator you would like [&,|, <=shift left or >=shift right");
    read(&mut operator);

    let num1: i64 =num1.trim().parse().unwrap();// conver string to integer 
    let num2: i64 =num2.trim().parse().unwrap();
    let operator: char =operator.trim().parse().unwrap();

    let operators = String:: from("[&|<>]");

    if !operators.contains(operator){
        println!("Unknown operators");
    }

    let result= match operator{ 
    '&'=> num1 & num2,
    '|'=> num1 | num2,
    '<'=> num1 << num2,
    '>'=> num1 >> num2,
     _=> panic!("Error in Operator")
    };
   
    println!("The result is: {} {} {} ={}",num1,operator,num2,result);
    println!("------------End-------------- ");


}

fn negate_calculator(){

    println!("Negate Calculator");
    let mut num1 = String::new();         //take all i/p as string 

    println!("Enter  Digit:"); // read  i/p to as string 
    read(&mut num1);

    let num1: i8 =num1.trim().parse().unwrap();// conver string to integer 

    let result = !num1;
   
    println!("The result is: ! {} ={}",num1,result);
    println!("------------End-------------- ");
}