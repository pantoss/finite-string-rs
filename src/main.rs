
struct FiniteString<const MIN: usize, const MAX: usize>(String);

impl<MIN, MAX> FiniteString
{  
    pub fn new(s &str)  -> Result<Self, String>{
    if s.len() < MIN {
        return Err("MIN > MAX".to_string())
    }
    else if s.len() > MAX {
        return Err("MIN > MAX".to_string())
    }
    Ok(FiniteString { data: s.to_string() })
}
}

fn main() {
    let s = FiniteString::<6, 20>::new("strin").unwrap();
    println!("{}", s.data);
}
