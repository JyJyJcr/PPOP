use std::{any::TypeId, fmt::Debug, marker::PhantomData, ops};

use anyhow::anyhow;

use crate::{agent::Agent, deduct::AgentPrecursor};

use super::{YBuildable, YOp};

pub struct Add<T1, T2> {
    t1: PhantomData<T1>,
    t2: PhantomData<T2>,
}
impl<T1, T2> Debug for Add<T1, T2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Add")
            .field("t1", &self.t1)
            .field("t2", &self.t2)
            .finish()
    }
}

impl<T1: 'static, T2: 'static, O: 'static> YOp for Add<T1, T2>
where
    for<'a, 'b> &'a T1: ops::Add<&'b T2, Output = O>,
{
    type Input1 = T1;

    type Input2 = T2;

    type Output = O;

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            t1: PhantomData,
            t2: PhantomData,
        })
    }

    fn exec(
        &self,
        e1: &Self::Input1,
        e2: &Self::Input2,
    ) -> impl IntoIterator<Item = <Self as YOp>::Output> {
        [e1 + e2]
    }
}

#[derive(Debug)]
pub struct SSAdd {}
impl YOp for SSAdd {
    type Input1 = String;

    type Input2 = String;

    type Output = String;

    fn new() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {})
    }

    fn exec(
        &self,
        e1: &Self::Input1,
        e2: &Self::Input2,
    ) -> impl IntoIterator<Item = <Self as YOp>::Output> {
        [format!("{}{}", e1, e2)]
    }
}

pub fn precursor(
    pi1: &str,
    pi2: &str,
    po: &str,
) -> anyhow::Result<Box<dyn AgentPrecursor<String>>> {
    let pi1 = pi1.to_string();
    let pi2 = pi2.to_string();
    let po = po.to_string();
    Ok(Box::new(AddPrecursor { pi1, pi2, po }))
}

#[derive(Debug)]
struct AddPrecursor {
    pi1: String,
    pi2: String,
    po: String,
}
impl AgentPrecursor<String> for AddPrecursor {
    fn deduct(&self, idx: &mut crate::deduct::PipeTypeIndex<String>) -> anyhow::Result<()> {
        let t1 = idx.ask(&self.pi1);
        let t2 = idx.ask(&self.pi2);
        //let to = idx.ask(&self.po);
        if let (Some(t1), Some(t2)) = (t1, t2) {
            if *t1 == TypeId::of::<String>() && *t2 == TypeId::of::<String>() {
                //idx.require::<_, String>(&self.pi1);
                //idx.require::<_, String>(&self.pi2);
                idx.require::<String, _>(&self.po)?;
            }
        }

        // let t: Option<&TypeId>=match (t1, t2, to) {
        //     (None, None, None) => None,
        //     (Some(t1), None, None) => Some(t1),
        //     (None, Some(t2), None) => Some(t2),
        //     (None, None, Some(to)) => Some(to),
        //     (Some(t1), Some(t2), None) => todo!(),
        //     (Some(_), None, Some(_)) => todo!(),
        //     (None, Some(_), Some(_)) => todo!(),
        //     (Some(_), Some(_), Some(_)) => todo!(),
        // }
        // match t {
        //     Some(_) => todo!(),
        //     None => todo!(),
        // }
        Ok(())
    }

    fn build(
        self: Box<Self>,
        idx: &crate::deduct::PipeIndex<String>,
    ) -> anyhow::Result<Box<dyn Agent>> {
        let t1 = idx.ask(&self.pi1)?;
        let t2 = idx.ask(&self.pi2)?;
        let to = idx.ask(&self.po)?;
        if t1 == TypeId::of::<String>()
            && t2 == TypeId::of::<String>()
            && to == TypeId::of::<String>()
        {
            return Ok(Box::new(SSAdd::build(self.pi1, self.pi2, self.po, idx)?));
        }

        Err(anyhow!("Add implement is WIP"))
    }
}

// pub fn build(
//     pi1: &str,
//     pi2: &str,
//     po: &str,
//     board: &mut PipeBoard<String>,
// ) -> anyhow::Result<Box<dyn Agent>> {
//     let t1 = board.get_type(pi1);
//     let t2 = board.get_type(pi2);
//     let to = board.get_type(po);

//     match (t1, t2, to) {
//         (Some(t), _, _) => {
//             infer!(pi1, pi2, po, board, SSAdd, t, String);
//             infer!(pi1, pi2, po, board, Add<u8,u8>, t, u8);
//             infer!(pi1, pi2, po, board, Add<i64,i64>, t, i64);
//             infer!(pi1, pi2, po, board, Add<u64,u64>, t, u64);
//             infer!(pi1, pi2, po, board, Add<f64,f64>, t, f64);
//             infer!(pi1, pi2, po, board, Add<isize,isize>, t, isize);
//             infer!(pi1, pi2, po, board, Add<usize,usize>, t, usize);
//         }
//         (None, Some(t), _) => {
//             infer!(pi1, pi2, po, board, SSAdd, t, String);
//             infer!(pi1, pi2, po, board, Add<u8,u8>, t, u8);
//             infer!(pi1, pi2, po, board, Add<i64,i64>, t, i64);
//             infer!(pi1, pi2, po, board, Add<u64,u64>, t, u64);
//             infer!(pi1, pi2, po, board, Add<f64,f64>, t, f64);
//             infer!(pi1, pi2, po, board, Add<isize,isize>, t, isize);
//             infer!(pi1, pi2, po, board, Add<usize,usize>, t, usize);
//         }
//         (None, None, Some(t)) => {
//             infer!(pi1, pi2, po, board, SSAdd, t, String);
//             infer!(pi1, pi2, po, board, Add<u8,u8>, t, u8);
//             infer!(pi1, pi2, po, board, Add<i64,i64>, t, i64);
//             infer!(pi1, pi2, po, board, Add<u64,u64>, t, u64);
//             infer!(pi1, pi2, po, board, Add<f64,f64>, t, f64);
//             infer!(pi1, pi2, po, board, Add<isize,isize>, t, isize);
//             infer!(pi1, pi2, po, board, Add<usize,usize>, t, usize);
//         }
//         (None, None, None) => infer!(pi1, pi2, po, board, SSAdd),
//     }
//     infer_fail!();
// }
