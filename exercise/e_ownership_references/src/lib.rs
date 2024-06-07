
pub fn inspect(s: &String)
{
    let check:bool;
    check = s.ends_with("s");

    if check == true
    {
        println!("Plural");
    }
    else{
        println!("Singular");
    }   
}

pub fn change(s: &mut String)
{
    let check:bool;
    check = s.ends_with("s");

    if check == true
    {

    }
    else
    {
        s.push_str("s");
    }
}

pub fn eat(s:String) -> bool
{
    if s.starts_with("b") && s.contains("a")
    {
        true
    }
    else{
        false
    }
}