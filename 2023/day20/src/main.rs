use core::fmt;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io,
};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Pulse {
    High,
    Low,
}

trait Module: fmt::Debug {
    fn name(&self) -> &str;
    fn inputs(&self) -> &HashSet<String>;
    fn destinations(&self) -> &Vec<String>;
    fn wire_input(&mut self, input: &str);

    fn handle_pulse(&mut self, from: String, pulse: Pulse) -> Vec<(String, Pulse, String)>;
}

fn broadcast<M: Module>(m: &M, pulse: Pulse) -> Vec<(String, Pulse, String)> {
    m.destinations()
        .iter()
        .map(|d| (m.name().to_string(), pulse, d.clone()))
        .collect()
}

#[derive(Debug)]
struct OutputModule {
    name: String,
    inputs: HashSet<String>,
    destinations: Vec<String>,
}

impl OutputModule {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            inputs: HashSet::new(),
            destinations: vec![],
        }
    }
}

impl Module for OutputModule {
    fn name(&self) -> &str {
        &self.name
    }
    fn inputs(&self) -> &HashSet<String> {
        &self.inputs
    }
    fn destinations(&self) -> &Vec<String> {
        &self.destinations
    }
    fn wire_input(&mut self, input: &str) {
        self.inputs.insert(input.to_string());
    }
    fn handle_pulse(&mut self, _from: String, _pulse: Pulse) -> Vec<(String, Pulse, String)> {
        vec![]
    }
}

// prefix %
#[derive(Debug)]
struct FlipFlopModule {
    name: String,
    on: bool,
    inputs: HashSet<String>,
    destinations: Vec<String>,
}

impl FlipFlopModule {
    fn new(name: &str, destinations: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            on: false,
            inputs: HashSet::new(),
            destinations,
        }
    }
}

impl Module for FlipFlopModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn inputs(&self) -> &HashSet<String> {
        &self.inputs
    }

    fn destinations(&self) -> &Vec<String> {
        &self.destinations
    }

    fn wire_input(&mut self, input: &str) {
        self.inputs.insert(input.to_string());
    }

    fn handle_pulse(&mut self, _from: String, pulse: Pulse) -> Vec<(String, Pulse, String)> {
        match pulse {
            Pulse::High => vec![],
            Pulse::Low => match self.on {
                true => {
                    self.on = false;
                    broadcast(self, Pulse::Low)
                }
                false => {
                    self.on = true;
                    broadcast(self, Pulse::High)
                }
            },
        }
    }
}

// prefix &
#[derive(Debug)]
struct ConjunctionModule {
    name: String,
    memory: HashMap<String, Pulse>,
    inputs: HashSet<String>,
    destinations: Vec<String>,
}

impl ConjunctionModule {
    fn new(name: &str, destinations: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            memory: HashMap::new(),
            inputs: HashSet::new(),
            destinations,
        }
    }
}

impl Module for ConjunctionModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn inputs(&self) -> &HashSet<String> {
        &self.inputs
    }

    fn destinations(&self) -> &Vec<String> {
        &self.destinations
    }

    fn wire_input(&mut self, input: &str) {
        self.inputs.insert(input.to_string());
        self.memory.insert(input.to_string(), Pulse::Low);
    }

    fn handle_pulse(&mut self, from: String, pulse: Pulse) -> Vec<(String, Pulse, String)> {
        *self.memory.get_mut(&from).unwrap() = pulse;
        if self.memory.values().all(|p| *p == Pulse::High) {
            broadcast(self, Pulse::Low)
        } else {
            broadcast(self, Pulse::High)
        }
    }
}

#[derive(Debug)]
struct BroadcastModule {
    name: String,
    inputs: HashSet<String>,
    destinations: Vec<String>,
}

impl BroadcastModule {
    fn new(destinations: Vec<String>) -> Self {
        Self {
            name: "broadcaster".to_string(),
            inputs: HashSet::new(),
            destinations,
        }
    }
}

impl Module for BroadcastModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn inputs(&self) -> &HashSet<String> {
        &self.inputs
    }

    fn destinations(&self) -> &Vec<String> {
        &self.destinations
    }

    fn wire_input(&mut self, input: &str) {
        self.inputs.insert(input.to_string());
    }

    fn handle_pulse(&mut self, _from: String, pulse: Pulse) -> Vec<(String, Pulse, String)> {
        broadcast(self, pulse)
    }
}

fn parse(line: &str) -> Box<dyn Module> {
    let t = line.split(" -> ").collect::<Vec<&str>>();

    let destinations = t[1]
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<_>>();

    let m: Box<dyn Module> = if t[0] == "broadcaster" {
        Box::new(BroadcastModule::new(destinations))
    } else {
        match &t[0][0..1] {
            "%" => Box::new(FlipFlopModule::new(&t[0][1..], destinations)),
            "&" => Box::new(ConjunctionModule::new(&t[0][1..], destinations)),
            _ => panic!("Unknown module type"),
        }
    };
    m
}

fn wire_inputs(modules: &mut HashMap<String, Box<dyn Module>>) {
    modules
        .iter()
        .flat_map(|(name, m)| m.destinations().iter().map(|d| (d.clone(), name.clone())))
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|(input, dest)| {
            modules
                .entry(input.clone())
                .or_insert_with(|| Box::new(OutputModule::new(&input)))
                .wire_input(&dest);
        });
}

struct Machine {
    presses: u64,
    modules: HashMap<String, Box<dyn Module>>,
    needed: HashMap<String, Option<u64>>,
}

impl Machine {
    fn new(modules: HashMap<String, Box<dyn Module>>, destination: &str) -> Self {
        let last_inputs = modules.get(destination).unwrap().inputs();
        assert_eq!(last_inputs.len(), 1);
        let last_input = last_inputs.iter().next().unwrap();
        let needed = modules
            .get(last_input)
            .unwrap()
            .inputs()
            .iter()
            .map(|i| (i.clone(), None))
            .collect();
        Self {
            presses: 0,
            modules,
            needed,
        }
    }

    fn got_all_needed(&self) -> bool {
        self.needed.values().all(|v| v.is_some())
    }

    fn mark_needed(&mut self, name: &str, pulse: Pulse) {
        if pulse != Pulse::High {
            return;
        }
        if let Some(n) = self.needed.get_mut(name) {
            if n.is_none() {
                *n = Some(self.presses);
            }
        }
    }

    fn press_button(&mut self) -> (u64, u64) {
        self.presses += 1;
        let mut low_signal_count = 1u64;
        let mut high_signal_count = 0u64;
        let mut todo =
            VecDeque::from([("button".to_string(), Pulse::Low, "broadcaster".to_string())]);
        while let Some((from, pulse, name)) = todo.pop_front() {
            self.mark_needed(&from, pulse);

            let m = self.modules.get_mut(&name).unwrap();
            let signals = m.handle_pulse(from, pulse);
            low_signal_count += signals.iter().filter(|(_, p, _)| *p == Pulse::Low).count() as u64;
            high_signal_count +=
                signals.iter().filter(|(_, p, _)| *p == Pulse::High).count() as u64;
            todo.extend(signals);
        }
        (low_signal_count, high_signal_count)
    }
}

fn main() {
    let mut modules = io::stdin()
        .lines()
        .map(|l| {
            let m = parse(l.unwrap().as_str());
            (m.name().to_string(), m)
        })
        .collect::<HashMap<_, _>>();
    wire_inputs(&mut modules);

    let mut machine = Machine::new(modules, "rx");
    let (l, h) = (0..1000)
        .map(|_| machine.press_button())
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d));
    let r1 = l * h;
    println!("{}", r1);

    while !machine.got_all_needed() {
        machine.press_button();
    }
    let r2 = machine.needed.values().map(|v| v.unwrap()).product::<u64>();
    println!("{}", r2);
}
