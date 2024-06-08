use super::bite::Bite;

#[derive(Debug)]
pub struct Grapes{
    pub amount_left:i32,
}

impl Bite for Grapes{
    fn bite(self:&mut Self)
    {
        self.amount_left -= 1;
    }
}