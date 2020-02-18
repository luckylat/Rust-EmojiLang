
use std::io;
use std::str;
/*
言語仕様
🤢:>
🤡:<
🤖:+
🧠:-
🥶:.
🤯:,
👹:[
👿:]
*/
fn main() {
    println!("code");
    let mut nya = String::new();
    io::stdin().read_line(&mut nya).expect("Failed.");
    let mut cin:usize = 0;
    let mut bf:Vec<char> = Vec::new();
    for (_i,c) in nya.chars().enumerate(){
        match c {
            '🤢' => {
                bf.push('>');
            }
            '🤡' => {
                bf.push('<');
            }
            '🤖' => {
                bf.push('+');
            }
            '🧠' => {
                bf.push('-');
            }
            '🥶' => {
                bf.push('.');
            }
            '🤯' => {
                bf.push(',');
                cin+=1;
            }
            '👹' => {
                bf.push('[');
            }
            '👿' => {
                bf.push(']');
            }
            _ => {}
        }
    }
    if cin != 0 {
        println!("Input data");
    }
    let mut nyi = String::new();
    let mut tmp = String::new();
    while cin > nyi.chars().count() {
        io::stdin().read_line(&mut tmp).expect("Failed.");
        for (_i,c) in tmp.chars().enumerate(){nyi.push(c)};
    }
    let mut mem:Vec<i8> = Vec::new();
    mem.push(0);
    let mut poi:usize = 0;
    let mut nan:usize = 0;
    let mut nynw:usize = 0;
    let mut i:usize = 0;
    let mut stack:Vec<usize> = Vec::new();
    while i < bf.len() {
        match bf[i] {
            '>' => {
                if mem.len() == poi+1 {
                    mem.push(0);
                }
                poi+=1;
            }
            '<' => {
                if poi != 0 {
                    poi-=1;
                }
            }
            '+' => {
                mem[poi]+=1;
            }
            '-' => {
                mem[poi]-=1;
            }
            '.' => {
                print!("{}",mem[poi]);
            }
            ',' => {
                mem[poi] = nyi.chars().nth(nynw).unwrap() as i8;
                nynw+=1;
            }
            '[' => {
                stack.push(i);
            }
            ']' => {
                if mem[poi] != 0{
                    i = stack[stack.len()-1];
                }else{
                    stack.pop();
                }
            }
            _ => {}
        }


        i+=1;
    }

    print!("[end]\n");
}