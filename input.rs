use std::io;

///will store the input in the given argument
/// 
///needs a mutable reference to that variable
/// 
///works like scanf() from C
/// 
///for example getstr(&mut x) will store the user input into x
pub fn getstr(writeto: &mut String)
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Not a valid input");
    *writeto = input;
}

///will store the input in the given argument
/// 
///needs a mutable reference to that variable
/// 
///works like scanf() from C
/// 
///for example getnum::i32(&mut x) will store the user input into x
pub mod getnum
{
    use std::io;
    
    ///for explanation look at getnum
    pub fn i128(writeto: &mut i128)
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Not a valid input");
        let number = input.trim().parse().expect("Not a valid input");
        *writeto = number;
    }

    ///for explanation look at getnum
    pub fn i64(writeto: &mut i64)
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Not a valid input");
        let number = input.trim().parse().expect("Not a valid input");
        *writeto = number;
    }

    ///for explanation look at getnum
    pub fn i32(writeto: &mut i32)
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Not a valid input");
        let number = input.trim().parse().expect("Not a valid input");
        *writeto = number;
    }

    ///for explanation look at getnum
    pub fn i16(writeto: &mut i16)
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Not a valid input");
        let number = input.trim().parse().expect("Not a valid input");
        *writeto = number;
    }

    ///for explanation look at getnum
    pub fn i8(writeto: &mut i8)
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Not a valid input");
        let number = input.trim().parse().expect("Not a valid input");
        *writeto = number;
    }

    ///for explanation look at getnum
    pub fn f64(writeto: &mut f64)
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Not a valid input");
        let number = input.trim().parse().expect("Not a valid input");
        *writeto = number;
    }

    ///for explanation look at getnum
    pub fn f32(writeto: &mut f32)
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Not a valid input");
        let number = input.trim().parse().expect("Not a valid input");
        *writeto = number;
    }
}
