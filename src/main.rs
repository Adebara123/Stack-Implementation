
fn new_stack(maxsize: usize) -> Vec<u32> {
    let vec: Vec<u32> = Vec::with_capacity(maxsize);
    vec
}

fn pop (stack: &mut Vec<u32>) -> Option<u32> {
    let poped_value = stack.pop();
    println!("The poped value is {:?}", poped_value);
    poped_value
}

fn push (stack: &mut Vec<u32>, item: u32, maxsize: usize) {
    if stack.len() > maxsize {
        println!("Can not add more");
    }
    else {
        stack.push(item);
        println!("Stack: {:?}", stack);
    }
}

fn size (stack: &Vec<u32>) -> usize {
    stack.len()
}

fn input () -> u32 {

    let mut _input = String::new();
    std::io::stdin()
    .read_line(&mut _input)
    .expect("Failed to read input");

    let _input = _input.trim().parse().expect("Wrong input");
    _input
}
 


fn main() {
   
   println!("Let's create a stack for our use");
   println!("Enter the size of the stack to be created");
   let stack_size = input();

   let mut stack = new_stack(stack_size as usize);
   

    loop {

    

   println!("\n\n  ************ Menu ************ \n");
   println!("1. Push \n 2. Pop \n 3.  Display \n 4. Size  \n 5. Exit");
   println!("\n Enter your choice: ");
   let choice = input();

   match choice {
    1 => {
        println!("Enter the value to be inserted: ");
        let item = input();
        push(&mut stack, item, stack_size as usize);
    }

    2 => {
        println!("The element popped out is {:?}", pop(&mut stack))
    }

    3 => {
        println!("The elements are {:?}", stack);
    }

    4 => {
        println!("The size of the stack is {}", size(&stack));
    }

    5 => break,
    
    _ => println!("Wrong selection"),
   }


   println!("Do you want to continue 1. = Yes  / 0. = No");
   let status = input();
   if status == 1  {
    continue;
   } else {
    break;
   }
 }

}
