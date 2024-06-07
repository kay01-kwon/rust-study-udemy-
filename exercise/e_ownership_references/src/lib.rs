
pub fn inspect(s: &String)
{
    let check:bool;
    check = s.ends_with("s");

    if check == true
    {
        println!("{} is plural",s);
    }
    else{
        println!("{} is singular",s);
    }   
}

pub fn change(s: &mut String)
{
    let check:bool;
    check = s.ends_with("s");

    if check == false
    {
        s.push_str("s");
    }

}

pub fn eat(s:String) -> bool
{
    s.starts_with("b") && s.contains("a")
}

pub fn bedazzle(s:&mut String) -> &mut String
{
    *s = String::from("sparkly");
    s
}