mod agent;
mod op;
mod op_imm;
mod operator;
mod pipe;

use std::{
    collections::{HashMap, HashSet},
    env::args,
    fs::read_to_string,
    hash::Hash,
    path::PathBuf,
};

use anyhow::{anyhow, Context};
use op::{ImmOperator, MergeOperator, Operator};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq, Eq)]
enum PipeStatus {
    ALIVE,
    ZOMBIE,
    DEAD,
}

#[derive(Debug)]
struct PipeEntry<PID> {
    id: PID,
}
impl<PID: Eq + Hash> PipeEntry<PID> {
    fn translate_id<NPID: Clone>(self, dic: &HashMap<PID, NPID>) -> PipeEntry<NPID> {
        PipeEntry {
            id: dic.get(&self.id).unwrap().clone(),
        }
    }
}
impl<PID> From<PID> for PipeEntry<PID> {
    fn from(value: PID) -> Self {
        Self { id: value }
    }
}
impl PipeEntry<usize> {
    fn cut<AID: Eq + Hash>(&self, aid: &AID, pipe_vec: &mut Vec<Option<Pipe<AID>>>) {
        match &mut pipe_vec[self.id] {
            Some(pipe) => {
                pipe.providers.remove(aid);
            }
            None => (),
        }
    }
    fn put<AID>(&self, e: Element, pipe_vec: &mut Vec<Option<Pipe<AID>>>) {
        match &mut pipe_vec[self.id] {
            Some(pipe) => {
                pipe.queue.push(e);
            }
            None => (),
        }
    }
}

#[derive(Debug)]
struct PipePointer<PID> {
    id: PID,
    loc: usize,
}
impl<PID: Eq + Hash> PipePointer<PID> {
    fn translate_id<NPID: Clone>(self, dic: &HashMap<PID, NPID>) -> PipePointer<NPID> {
        PipePointer {
            id: dic.get(&self.id).unwrap().clone(),
            loc: self.loc,
        }
    }
}
impl PipePointer<usize> {
    fn has_next<AID>(&self, pipe_vec: &Vec<Option<Pipe<AID>>>) -> bool {
        if let Some(Some(pipe)) = pipe_vec.get(self.id) {
            pipe.queue.len() > self.loc
        } else {
            false
        }
    }
    fn next<'x, AID>(&mut self, pipe_vec: &'x Vec<Option<Pipe<AID>>>) -> &'x Element {
        match pipe_vec.get(self.id).unwrap() {
            Some(pipe) => {
                let e = pipe.queue.get(self.loc).unwrap();
                self.loc += 1;
                return e;
            }
            None => (),
        }
        panic!("logic err");
    }

    fn is_dead<AID>(&self, pipe_vec: &Vec<Option<Pipe<AID>>>) -> bool {
        if let Some(Some(p)) = pipe_vec.get(self.id) {
            !(p.status() == PipeStatus::ALIVE && self.has_next(pipe_vec))
        } else {
            true
        }
    }
    fn cut<AID: Eq + Hash>(&self, aid: &AID, pipe_vec: &mut Vec<Option<Pipe<AID>>>) {
        match &mut pipe_vec[self.id] {
            Some(pipe) => {
                pipe.consumers.remove(aid);
            }
            None => (),
        }
    }
}

impl<PID> From<PID> for PipePointer<PID> {
    fn from(value: PID) -> Self {
        Self { id: value, loc: 0 }
    }
}

#[derive(Debug)]
enum Agent<AID, PID> {
    IMM(ImmAgent<AID, PID>),
    MRG(MergeAgent<AID, PID>),
}
#[derive(Debug)]
struct ImmAgent<AID, PID> {
    id: AID,
    pi: PipePointer<PID>,
    imm: String,
    op: ImmOperator,
    po: PipeEntry<PID>,
}
#[derive(Debug)]
struct MergeAgent<AID, PID> {
    id: AID,
    pi1: PipePointer<PID>,
    pi2: PipePointer<PID>,
    op: MergeOperator,
    po: PipeEntry<PID>,
}
impl<AID, PID: Eq + Hash> ImmAgent<AID, PID> {
    fn translate_id<NPID: Clone>(self, dic: &HashMap<PID, NPID>) -> ImmAgent<AID, NPID> {
        ImmAgent {
            id: self.id,
            pi: self.pi.translate_id(dic),
            imm: self.imm,
            op: self.op,
            po: self.po.translate_id(dic),
        }
    }
}
impl<AID, PID: Eq + Hash> MergeAgent<AID, PID> {
    fn translate_id<NPID: Clone>(self, dic: &HashMap<PID, NPID>) -> MergeAgent<AID, NPID> {
        MergeAgent {
            id: self.id,
            pi1: self.pi1.translate_id(dic),
            pi2: self.pi2.translate_id(dic),
            op: self.op,
            po: self.po.translate_id(dic),
        }
    }
}

impl<AID: Eq + Hash> ImmAgent<AID, usize> {
    fn update(&mut self, pipe_vec: &mut Vec<Option<Pipe<AID>>>) -> AgentStatus {
        if self.pi.has_next(pipe_vec) {
            let e = self.op.exec(self.pi.next(pipe_vec), &self.imm);
            match e {
                Some(e) => {
                    self.po.put(e, pipe_vec);
                }
                None => (),
            }
        }
        if self.pi.is_dead(pipe_vec) {
            self.pi.cut(&self.id, pipe_vec);
            self.po.cut(&self.id, pipe_vec);
            AgentStatus::DEAD
        } else {
            AgentStatus::ALIVE
        }
    }
}

impl<AID: Eq + Hash> MergeAgent<AID, usize> {
    fn update(&mut self, pipe_vec: &mut Vec<Option<Pipe<AID>>>) -> AgentStatus {
        if self.pi1.has_next(pipe_vec) && self.pi2.has_next(pipe_vec) {
            let e = self
                .op
                .exec(self.pi1.next(pipe_vec), &self.pi2.next(pipe_vec));
            match e {
                Some(e) => {
                    self.po.put(e, pipe_vec);
                }
                None => (),
            }
        }
        if self.pi1.is_dead(pipe_vec) || self.pi2.is_dead(pipe_vec) {
            self.pi1.cut(&self.id, pipe_vec);
            self.pi2.cut(&self.id, pipe_vec);
            self.po.cut(&self.id, pipe_vec);
            AgentStatus::DEAD
        } else {
            AgentStatus::ALIVE
        }
    }
}

enum AgentStatus {
    ALIVE,
    DEAD,
}

impl<AID, PID: Eq + Hash> Agent<AID, PID> {
    fn translate_id<NPID: Clone>(self, dic: &HashMap<PID, NPID>) -> Agent<AID, NPID> {
        match self {
            Agent::IMM(ag) => Agent::IMM(ag.translate_id(dic)),
            Agent::MRG(ag) => Agent::MRG(ag.translate_id(dic)),
        }
    }
}

impl<AID: Eq + Hash> Agent<AID, usize> {
    fn update(&mut self, pipe_vec: &mut Vec<Option<Pipe<AID>>>) -> AgentStatus {
        match self {
            Agent::IMM(ag) => ag.update(pipe_vec),
            Agent::MRG(ag) => ag.update(pipe_vec),
        }
    }
}

#[derive(Debug)]
struct Pipe<AID> {
    queue: Vec<Element>,
    providers: HashSet<AID>,
    consumers: HashSet<AID>,
}

#[derive(Debug, Clone)]
enum Element {
    STR(String),
    UINT(u64),
    INT(i64),
    FLOAT(f64),
    BOOL(bool),
}

impl<AID> Pipe<AID> {
    fn new() -> Self {
        Self {
            queue: Vec::new(),
            providers: HashSet::new(),
            consumers: HashSet::new(),
        }
    }
    fn status(&self) -> PipeStatus {
        use PipeStatus::*;
        if self.providers.is_empty() {
            if self.consumers.is_empty() {
                DEAD
            } else {
                ZOMBIE
            }
        } else {
            ALIVE
        }
    }
}

// struct PipeIdx{
//     vec: Vec<Pipe>,
// }
// impl PipeIdx{
//     fn get
// }
// struct PipeVecBuilder{
//     idx: HashMap<String, usize>,
//     vec: Vec<Pipe>,
// }
// impl PipeVecBuilder{
//     fn new()->Self{
//         Self{
//             idx:HashMap::new(),
//             vec:Vec::new(),
//         }
//     }
//     fn register(&mut self,name:&str,aid:usize)->usize{
//         match self.idx.get(name) {
//             Some(i) => *i,
//             None => {
//                 self.vec.push(Pipe::new());
//                 self.vec.len()-1
//             },
//         }
//     }
//     fn build(self)->PipeIdx{
//         self.pipe_idx
//     }
// }

// struct PipeIdx<AID> {
//     vec: Vec<Option<Pipe>>,
// }
// impl PipeIdx {
//     fn is_alive(&self, id: usize) -> bool {
//         self.vec[id].is_some()
//     }
// }

struct PipeVecBuilder<PID: Eq + Hash, AID: Eq + Hash> {
    idx: HashMap<PID, Pipe<AID>>,
}
impl<PID: Eq + Hash, AID: Eq + Hash> PipeVecBuilder<PID, AID> {
    fn new() -> Self {
        Self {
            idx: HashMap::new(),
        }
    }

    fn register_provider(&mut self, pipe_id: PID, provider_id: AID) {
        match self.idx.get_mut(&pipe_id) {
            Some(pipe) => {
                pipe.providers.insert(provider_id);
            }
            None => {
                let mut pipe = Pipe::new();
                pipe.providers.insert(provider_id);
                self.idx.insert(pipe_id, pipe);
            }
        }
    }
    fn register_consumer(&mut self, pipe_id: PID, consumer_id: AID) {
        match self.idx.get_mut(&pipe_id) {
            Some(pipe) => {
                pipe.consumers.insert(consumer_id);
            }
            None => {
                let mut pipe = Pipe::new();
                pipe.consumers.insert(consumer_id);
                self.idx.insert(pipe_id, pipe);
            }
        }
    }

    fn build(self) -> (Vec<Option<Pipe<AID>>>, HashMap<PID, usize>) {
        let mut dic: HashMap<PID, usize> = HashMap::new();
        let mut counter = 0;
        let vec = self
            .idx
            .into_iter()
            .map(|(id, pipe)| {
                dic.insert(id, counter);
                counter += 1;
                Some(pipe)
            })
            .collect();
        (vec, dic)
    }
}

// fn check_alive(pidx:&PipeIdx,agent:&Agent)->bool{
//     match agent {
//         Agent::IMM(ag) => {
//             pidx.is_alive(ag.pi)
//         },
//         Agent::MRG(ag) => {
//             pidx.is_alive(ag.pi1)&&pidx.is_alive(ag.pi2)
//         },
//     }
// }
// fn check_alive(aidx:&AgentIdx,pipe:&Pipe)->bool{
//     (!pipe.providers.is_empty())&&(!pipe.consumers.is_empty())
// }

fn main() -> anyhow::Result<()> {
    let it = args();
    let mut it = it.skip(1);
    let script: PathBuf = it
        .next()
        .ok_or(anyhow!("script file not specified"))?
        .into();
    let mut args: Vec<Element> = it.map(|s| Element::STR(s)).collect();
    println!("xxx {}", script.to_str().unwrap());

    let s = read_to_string(script).context(anyhow!("failed to read script"))?;
    let gv = s.graphemes(true).collect::<Vec<&str>>();
    if gv.len() % 4 != 0 {
        return Err(anyhow!("script alignment invalid"));
    }

    let mut pipe_vec_builder = PipeVecBuilder::new();
    let mut agent_vec: Vec<Agent<usize, &str>> = Vec::new();

    for i in 0..gv.len() / 4 {
        //parse
        let li1 = gv[4 * i];
        let li2 = gv[4 * i + 1];
        let lop = gv[4 * i + 2];
        let lo = gv[4 * i + 3];

        let op = Operator::from_grapheme(lop)?;

        let agent = match op {
            Operator::IMM(op) => {
                let pi = li1;
                let imm = li2.to_string();
                let po = lo;
                pipe_vec_builder.register_consumer(pi, i);
                pipe_vec_builder.register_provider(po, i);
                Agent::IMM(ImmAgent {
                    id: i,
                    pi: pi.into(),
                    imm,
                    op,
                    po: po.into(),
                })
            }
            Operator::MRG(op) => {
                let pi1 = li1;
                let pi2 = li2;
                let po = lo;
                pipe_vec_builder.register_consumer(pi1, i);
                pipe_vec_builder.register_consumer(pi2, i);
                pipe_vec_builder.register_provider(po, i);
                Agent::MRG(MergeAgent {
                    id: i,
                    pi1: pi1.into(),
                    pi2: pi2.into(),
                    op,
                    po: po.into(),
                })
            }
        };
        agent_vec.push(agent);
    }

    if let Some(p) = pipe_vec_builder.idx.get_mut("#") {
        p.queue.push(Element::UINT(args.len() as u64));
    }
    if let Some(p) = pipe_vec_builder.idx.get_mut("@") {
        p.queue.append(&mut args);
    }

    let (mut pipe_vec, pipe_dic) = pipe_vec_builder.build();
    let mut agent_vec: Vec<Option<Agent<usize, usize>>> = agent_vec
        .into_iter()
        .map(|agent| Some(agent.translate_id(&pipe_dic)))
        .collect();

    let mut counter: usize = 0;
    while is_alive_world(&pipe_vec, &agent_vec) && counter < 10 {
        println!("t={}", counter);
        for pipe in pipe_vec.iter() {
            println!("{:?}", pipe);
        }
        for agent in agent_vec.iter() {
            println!("{:?}", agent);
        }

        for oagent in agent_vec.iter_mut() {
            match oagent {
                Some(agent) => {
                    match agent.update(&mut pipe_vec) {
                        AgentStatus::ALIVE => (),
                        AgentStatus::DEAD => {
                            *oagent = None;
                        }
                    };
                }
                None => (),
            }
        }

        println!("t={} after agent update", counter);
        for pipe in pipe_vec.iter() {
            println!("{:?}", pipe);
        }
        for agent in agent_vec.iter() {
            println!("{:?}", agent);
        }

        // for agent in agent_vec.iter_mut() {
        //     agent.update();
        // }
        for opipe in pipe_vec.iter_mut() {
            match opipe {
                Some(pipe) => match pipe.status() {
                    PipeStatus::ALIVE => (),
                    PipeStatus::ZOMBIE => (),
                    PipeStatus::DEAD => *opipe = None,
                },
                None => (),
            }
        }

        println!("t={} after pipe update", counter);
        for pipe in pipe_vec.iter() {
            println!("{:?}", pipe);
        }
        for agent in agent_vec.iter() {
            println!("{:?}", agent);
        }

        counter += 1;
    }

    Ok(())
}

fn is_alive_world(
    pipe_vec: &Vec<Option<Pipe<usize>>>,
    agent_vec: &Vec<Option<Agent<usize, usize>>>,
) -> bool {
    let mut flag = false;
    for pipe in pipe_vec.iter() {
        if pipe.is_some() {
            flag = true;
        }
    }
    for agent in agent_vec.iter() {
        if agent.is_some() {
            flag = true;
        }
    }
    flag
}
