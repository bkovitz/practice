fn main() {
    let counts = count_events(vec![4, 2, 4, 3, 2, 4]);
    for (id, count) in counts {
        println!("({id}, {count})");
    }
}

fn count_events(events: Vec<u32>) -> Vec<(u32, u32)> {
    let mut result: Vec<(u32, u32)> = Vec::new();

    for id in events {
        match find_event_pair(&mut result, id) {
            Some(pair) => {
                pair.1 += 1;
            },
            None => result.push((id, 1)),
        }
    }
    result
}

fn find_event_pair(pairs: &mut Vec<(u32, u32)>, id: u32) -> Option<&mut (u32, u32)> {
    for i in 0..pairs.len() {
        if pairs[i].0 == id {
            return Some(&mut pairs[i]);
        }
    }
    None
}