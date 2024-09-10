use super::IOp;

#[derive(Debug)]
pub struct SxPrintf {}
impl IOp for SxPrintf {
    type Input = String;

    type Output = String;

    fn new(_: String) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }

    fn exec(&self, e: &Self::Input) -> impl IntoIterator<Item = Self::Output> {
        print!("{}", e);
        [e.clone()]
    }
}
