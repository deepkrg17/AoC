#[derive(Debug)]
struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}

impl Room {
    fn parse(input: &str) -> Self {
        let mut split = input.split('[');
        let mut splits = split.next().unwrap().rsplitn(2, '-');
        let sector_id = splits.next().unwrap().parse().unwrap();
        let name = splits.next().unwrap().to_string();
        let checksum = split.next().unwrap()[..5].to_string();
        Self {
            name,
            sector_id,
            checksum,
        }
    }

    fn is_real(&self) -> bool {
        let mut counts = ('a'..='z').map(|c| (0_u32, c)).collect::<Vec<_>>();
        for c in self.name.chars() {
            if c == '-' {
                continue;
            }
            counts[(c as u8 - b'a') as usize].0 += 1;
        }
        counts.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        counts[..5].iter().map(|(_, c)| *c).collect::<String>() == self.checksum
    }

    fn decrypt(&self) -> String {
        let mut decrypted = String::new();
        for c in self.name.chars() {
            if c == '-' {
                decrypted.push(' ');
                continue;
            }
            let c = (c as u8 - b'a') as u32;
            let c = ((c + self.sector_id) % 26) as u8;
            decrypted.push((c + b'a') as char);
        }
        decrypted
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let rooms: Vec<Room> = input
        .lines()
        .map(Room::parse)
        .filter(|r| r.is_real())
        .collect();

    println!("Part 1: {}", rooms.iter().map(|r| r.sector_id).sum::<u32>());

    println!(
        "Part 2: {}",
        rooms
            .iter()
            .find(|r| r.decrypt() == "northpole object storage")
            .unwrap()
            .sector_id
    );
}
