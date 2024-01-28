fn has_abba(input: &str) -> bool {
    input
        .as_bytes()
        .windows(4)
        .any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
}

fn get_abas(ips: &IpSplit) -> Vec<String> {
    let mut babs: Vec<String> = Vec::new();
    for s in ips.normals.iter() {
        babs.extend(
            s.as_bytes()
                .windows(3)
                .filter(|w| w[0] == w[2] && w[0] != w[1])
                .map(|w| {
                    let a = w[0] as char;
                    let b = w[1] as char;
                    format!("{b}{a}{b}")
                }),
        );
    }
    babs
}

struct IpSplit<'a> {
    normals: Vec<&'a str>,
    hypernets: Vec<&'a str>,
}

impl<'a> IpSplit<'a> {
    fn parse(ip: &'a str) -> Self {
        let mut normals = Vec::new();
        let mut hypernets = Vec::new();
        let split = ip.split(|c| c == '[' || c == ']');
        for (i, s) in split.enumerate() {
            if i % 2 == 0 {
                normals.push(s)
            } else {
                hypernets.push(s);
            }
        }
        Self { normals, hypernets }
    }

    fn supports_tls(&self) -> bool {
        self.normals.iter().any(|s| has_abba(s)) && self.hypernets.iter().all(|s| !has_abba(s))
    }

    fn supports_ssl(&self) -> bool {
        let babs = get_abas(self);
        self.hypernets
            .iter()
            .any(|s| babs.iter().any(|b| s.contains(b)))
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let ip_splits: Vec<IpSplit> = input.lines().map(IpSplit::parse).collect();

    println!(
        "TLS count: {}",
        ip_splits.iter().filter(|ips| ips.supports_tls()).count()
    );

    println!(
        "SSL count: {}",
        ip_splits.iter().filter(|ips| ips.supports_ssl()).count()
    );
}
