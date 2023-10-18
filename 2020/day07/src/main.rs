use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

use regex::Regex;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|l| l.unwrap());
    let re = Regex::new(r"^(\d+) ((\w|\s)+) bags?\.?$").unwrap();
    let mut bag_map = HashMap::<String, HashSet<String>>::new();
    let mut bag_map2 = HashMap::<String, Vec<(u32, String)>>::new();
    lines.for_each(|l| {
        let ll: Vec<_> = l.split(" bags contain ").collect();
        let bag = ll[0];
        let bags: Vec<_> = ll[1]
            .split(", ")
            .filter_map(|s| re.captures(s))
            .map(|m| {
                let count = m[1].parse::<u32>().unwrap();
                let color = m[2].to_string();
                (count, color)
            })
            .collect();
        bag_map2.insert(bag.to_string(), bags.clone());

        bags.iter()
            .for_each(|(_, c)| match bag_map.get_mut(c.as_str()) {
                Some(s) => {
                    s.insert(bag.to_string());
                }
                None => {
                    let mut s = HashSet::<String>::new();
                    s.insert(bag.to_string());
                    bag_map.insert(c.to_string(), s);
                }
            });
    });

    fn find_parent(
        color: &str,
        bag_map: &HashMap<String, HashSet<String>>,
        parents: &mut HashSet<String>,
    ) {
        if parents.contains(color) {
            return;
        }
        parents.insert(color.to_string());

        if let Some(bags) = bag_map.get(color) {
            bags.iter().for_each(|b| {
                find_parent(b, bag_map, parents);
            });
        }
    }

    fn count_children(color: &str, bag_map: &HashMap<String, Vec<(u32, String)>>) -> u32 {
        bag_map
            .get(color)
            .unwrap()
            .iter()
            .fold(0u32, |acc, (count, color)| {
                acc + count + count * count_children(color, bag_map)
            })
    }

    let mut bags = HashSet::<String>::new();
    find_parent("shiny gold", &bag_map, &mut bags);
    println!("{:?}", bags.len() - 1);
    println!("{}", count_children("shiny gold", &bag_map2));
}
