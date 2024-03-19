use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum ProtocolId {
    ProtocolUnknown = 0,
    ProtocolFtp = 1,
    ProtocolPop = 2,
    ProtocolSmtp = 3,
    ProtocolImap = 4,
    ProtocolDns = 5,
    ProtocolHttp = 6,
    ProtocolMdns = 7,
    ProtocolNtp = 8,
}

impl FromStr for ProtocolId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ftp" => Ok(Self::ProtocolFtp),
            "pop" => Ok(Self::ProtocolPop),
            "smtp" => Ok(Self::ProtocolSmtp),
            "imap" => Ok(Self::ProtocolImap),
            "dns" => Ok(Self::ProtocolDns),
            "http" => Ok(Self::ProtocolHttp),
            "mdns" => Ok(Self::ProtocolMdns),
            "ntp" => Ok(Self::ProtocolNtp),
            _ => Ok(Self::ProtocolUnknown),
        }
    }
}

fn convert(s: &str) -> Vec<ProtocolId> {
    let mut vec = Vec::new();
    for re in s.split(", ") {
        let id: ProtocolId = re.parse().expect("there are unexpected protocol, that we cannot handle, but how you got this...");
        vec.push(id);
    }
    vec
}

fn main() {
    let res = std::fs::read_to_string("file.txt").expect("we need file.txt");
    let vec = convert(res.as_str());
    println!("vec!{:?};", vec.into_iter().map(|e| e as i32).collect::<Vec<i32>>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_multiple_protocols() {
        let input = "dns, http, ntp";
        let expected_output = vec![ProtocolId::ProtocolDns, ProtocolId::ProtocolHttp, ProtocolId::ProtocolNtp];
        let output = convert(input);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_convert_with_https() {
        let input = "https, imap, pop";
        let expected_output = vec![ProtocolId::ProtocolUnknown, ProtocolId::ProtocolImap, ProtocolId::ProtocolPop];
        let output = convert(input);
        assert_eq!(output, expected_output);
    }
}