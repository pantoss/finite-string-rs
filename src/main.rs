
struct FiniteString<const MIN: usize, const MAX: usize>
{
    data : String
}

impl<const MIN: usize, const MAX: usize> FiniteString<MIN, MAX>
{  
    pub fn new(s :&str)  -> Result<Self, String>{
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
    let s = FiniteString::<6, 20>::new("stringstringstringstring").unwrap();
    println!("{}", s.data);
}
