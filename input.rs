use std::io;

pub fn getstr(writeto: &mut String)
{
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    *writeto = input;
}

pub mod getint
{
    use std::io;
    
    pub fn i128(writeto: &mut i128)
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let number = input.trim().parse().expect("Not a valid input");
        *writeto = number;
    }

    pub fn i64(writeto: &mut i64)
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let number = input.trim().parse().expect("Not a valid input");
        *writeto = number;
    }

    pub fn i32(writeto: &mut i32)
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let number = input.trim().parse().expect("Not a valid input");
        *writeto = number;
    }

    pub fn i16(writeto: &mut i16)
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let number = input.trim().parse().expect("Not a valid input");
        *writeto = number;
    }

    pub fn i(writeto: &mut i8)
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let number = input.trim().parse().expect("Not a valid input");
        *writeto = number;
    }
}