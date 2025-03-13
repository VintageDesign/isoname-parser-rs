#[repr(transparent)]
pub struct IsoName(u64);

impl IsoName{
    pub fn get_address_arb_capable(&self) -> u8
    {
        return self.raw_value >> 63;
    }

    pub fn get_industry_group(&self) -> u8
    {

    }
}
