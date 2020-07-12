use regex::Regex;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate lazy_static;

/*
V2rayN Vmess URI format:
from https://github.com/2dust/v2rayN/blob/master/v2rayN/v2rayN/Mode/VmessQRCode.cs
*/

lazy_static! {
    static ref VMESS_URL_MATCHER: Regex = Regex::new(r#"vmess://.*"#).unwrap();
    pub static ref DEFAULT_CONVERT_CFG: ConvertConfig = ConvertConfig {
        group: "V2NtoQuan".to_string(),
        method: "chacha20-ietf-poly1305".to_string(),
    };
}

#[derive(Clone)]
pub struct ConvertConfig {
    pub group: String,
    pub method: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VmessConfig {
    v: String,      // version
    ps: String,     // remark
    add: String,    // remote server address
    port: String,   // remote server port
    id: String,     // uid
    aid: String,    // alterid
    net: String,    // tcp/kcp/ws
    r#type: String, // obfs type, none/http
    host: String,   // obfs host
    path: String,   // obfs path
    tls: String,    // tls
}

impl VmessConfig {
    pub fn to_quan_uri(&self, convert_cfg: &ConvertConfig) -> String {
        let ua="Mozilla/5.0 (iPhone; CPU iPhone OS 11_2_6 like Mac OS X) AppleWebKit/604.5.6 (KHTML, like Gecko) Mobile/15D100";
        let obfs = format!(
            ", obfs={net}, obfs-path=\"{path}\", obfs-header=\"Host: {host}[Rr][Nn]User-Agent: {ua}\"",
            net = match self.net.as_ref() {
                "ws" => "ws",
                _ => "http",
            },
            path = if self.path.is_empty() {
                "/"
            } else {
                self.path.as_ref()
            },
            host = if self.host.is_empty() {
                &self.add
            } else {
                &self.host
            },
            ua = ua,
        );
        let quan_config_str = format!(
            "{remark} = vmess, {add}, {port}, {method}, \"{id}\", group={group}, over-tls={tls}, certificate=1{obfs}",
            remark = self.ps,
            add = self.add,
            port = self.port,
            method = convert_cfg.method,
            id = self.id,
            group = convert_cfg.group,
            tls = match self.tls.as_ref() {
                "tls" => "true",
                _ => "false",
            },
            obfs = match self.r#type.as_ref() {
                "none" => match self.net.as_ref() {
                    "ws" => obfs.as_ref(),
                    _ => "",
                },
                _=> obfs.as_ref(),
            }
        );
        let quan_config_encoded = base64::encode_config(&quan_config_str, base64::URL_SAFE);
        format!("vmess://{}", quan_config_encoded)
    }
}

pub fn convert_vmess_uri(ins: &str, convert_cfg: &ConvertConfig) -> String {
    let start = "vmess://".len();
    let serialized: String = String::from_utf8(base64::decode(&ins[start..]).unwrap()).unwrap();
    let deserialized: VmessConfig = serde_json::from_str(&serialized).unwrap();
    deserialized.to_quan_uri(convert_cfg)
}

pub fn convert_cfg_str(cfg_str: &str, convert_cfg: &ConvertConfig) -> String {
    let mut buffer = String::new();
    for line in cfg_str.lines() {
        if VMESS_URL_MATCHER.is_match(line) {
            buffer.push_str(&format!("{}\n", convert_vmess_uri(line, convert_cfg)));
        } else {
            buffer.push_str(line);
        }
    }
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let serialized = r#"{
            "v": "2",
            "ps": "IPOnlyV2N",
            "add": "123.23.3.12",
            "port": "1919",
            "id": "347e6167-68bf-40e2-a4ff-082f97ef6231",
            "aid": "233",
            "net": "tcp",
            "type": "none",
            "host": "",
            "path": "",
            "tls": ""
        }"#
        .to_string();
        let deserialized: VmessConfig = serde_json::from_str(&serialized).unwrap();
        let desired = r#"VmessConfig { v: "2", ps: "IPOnlyV2N", add: "123.23.3.12", port: "1919", id: "347e6167-68bf-40e2-a4ff-082f97ef6231", aid: "233", net: "tcp", type: "none", host: "", path: "", tls: "" }"#.to_string();
        assert_eq!(format!("{:?}", deserialized), desired);
    }

    fn check_convert(ins: &str, ous: &str) {
        assert_eq!(convert_vmess_uri(&ins, &DEFAULT_CONVERT_CFG), ous);
    }

    /*

    ** V2RayN Config **
    vmess://ew0KICAidiI6ICIyIiwNCiAgInBzIjogIklQT25seVYyTiIsDQogICJhZGQiOiAiMTIzLjIzLjMuMTIiLA0KICAicG9ydCI6ICIxOTE5IiwNCiAgImlkIjogIjM0N2U2MTY3LTY4YmYtNDBlMi1hNGZmLTA4MmY5N2VmNjIzMSIsDQogICJhaWQiOiAiMjMzIiwNCiAgIm5ldCI6ICJ0Y3AiLA0KICAidHlwZSI6ICJub25lIiwNCiAgImhvc3QiOiAiIiwNCiAgInBhdGgiOiAiIiwNCiAgInRscyI6ICIiDQp9

    {
        "v": "2",
        "ps": "IPOnlyV2N",
        "add": "123.23.3.12",
        "port": "1919",
        "id": "347e6167-68bf-40e2-a4ff-082f97ef6231",
        "aid": "233",
        "net": "tcp",
        "type": "none",
        "host": "",
        "path": "",
        "tls": ""
    }

    ** Quantumult Config**
    vmess://SVBPbmx5VjJOID0gdm1lc3MsIDEyMy4yMy4zLjEyLCAxOTE5LCBjaGFjaGEyMC1pZXRmLXBvbHkxMzA1LCAiMzQ3ZTYxNjctNjhiZi00MGUyLWE0ZmYtMDgyZjk3ZWY2MjMxIiwgb3Zlci10bHM9ZmFsc2UsIGNlcnRpZmljYXRlPTE=

    IPOnlyV2N = vmess, 123.23.3.12, 1919, chacha20-ietf-poly1305, "347e6167-68bf-40e2-a4ff-082f97ef6231", over-tls=false, certificate=1

    */

    #[test]
    fn convert_1() {
        let ins = r#"vmess://ew0KICAidiI6ICIyIiwNCiAgInBzIjogIklQT25seVYyTiIsDQogICJhZGQiOiAiMTIzLjIzLjMuMTIiLA0KICAicG9ydCI6ICIxOTE5IiwNCiAgImlkIjogIjM0N2U2MTY3LTY4YmYtNDBlMi1hNGZmLTA4MmY5N2VmNjIzMSIsDQogICJhaWQiOiAiMjMzIiwNCiAgIm5ldCI6ICJ0Y3AiLA0KICAidHlwZSI6ICJub25lIiwNCiAgImhvc3QiOiAiIiwNCiAgInBhdGgiOiAiIiwNCiAgInRscyI6ICIiDQp9"#;
        let ous = r#"vmess://SVBPbmx5VjJOID0gdm1lc3MsIDEyMy4yMy4zLjEyLCAxOTE5LCBjaGFjaGEyMC1pZXRmLXBvbHkxMzA1LCAiMzQ3ZTYxNjctNjhiZi00MGUyLWE0ZmYtMDgyZjk3ZWY2MjMxIiwgZ3JvdXA9VjJOdG9RdWFuLCBvdmVyLXRscz1mYWxzZSwgY2VydGlmaWNhdGU9MQ=="#;
        check_convert(&ins, &ous);
    }

    /*
    ** V2RayN Config **
    vmess://ew0KICAidiI6ICIyIiwNCiAgInBzIjogIlNPTUVWNiIsDQogICJhZGQiOiAiaXB2Ni5zZy53aG8ubW9lIiwNCiAgInBvcnQiOiAiNDA0IiwNCiAgImlkIjogIjY4YmI2MGUyLTJhMDQtNDk2Yy1iYjM4LWE0NDkyMDZhNGJkZSIsDQogICJhaWQiOiAiNjQiLA0KICAibmV0IjogIndzIiwNCiAgInR5cGUiOiAibm9uZSIsDQogICJob3N0IjogImlwdjYuc2cud2hvLm1vZSIsDQogICJwYXRoIjogIi93aGVyZS8iLA0KICAidGxzIjogInRscyINCn0=

    {
        "v": "2",
        "ps": "SOMEV6",
        "add": "ipv6.sg.who.moe",
        "port": "404",
        "id": "68bb60e2-2a04-496c-bb38-a449206a4bde",
        "aid": "64",
        "net": "ws",
        "type": "none",
        "host": "ipv6.sg.who.moe",
        "path": "/where/",
        "tls": "tls"
    }

    ** Quantumult Config**
    vmess://U09NRVY2ID0gdm1lc3MsIGlwdjYuc2cud2hvLm1vZSwgNDA0LCBjaGFjaGEyMC1pZXRmLXBvbHkxMzA1LCAiNjhiYjYwZTItMmEwNC00OTZjLWJiMzgtYTQ0OTIwNmE0YmRlIiwgb3Zlci10bHM9dHJ1ZSwgY2VydGlmaWNhdGU9MSwgb2Jmcz13cywgb2Jmcy1wYXRoPSIvd2hlcmUvIiwgb2Jmcy1oZWFkZXI9Ikhvc3Q6IGlwdjYuc2cud2hvLm1vZVtScl1bTm5dVXNlci1BZ2VudDogTW96aWxsYS81LjAgKGlQaG9uZTsgQ1BVIGlQaG9uZSBPUyAxMV8yXzYgbGlrZSBNYWMgT1MgWCkgQXBwbGVXZWJLaXQvNjA0LjUuNiAoS0hUTUwsIGxpa2UgR2Vja28pIE1vYmlsZS8xNUQxMDAi

    SOMEV6 = vmess, ipv6.sg.who.moe, 404, chacha20-ietf-poly1305, "68bb60e2-2a04-496c-bb38-a449206a4bde", over-tls=true, certificate=1, obfs=ws, obfs-path="/where/", obfs-header="Host: ipv6.sg.who.moe[Rr][Nn]User-Agent: Mozilla/5.0 (iPhone; CPU iPhone OS 11_2_6 like Mac OS X) AppleWebKit/604.5.6 (KHTML, like Gecko) Mobile/15D100"

    */

    #[test]
    fn convert_2() {
        let ins = r#"vmess://ew0KICAidiI6ICIyIiwNCiAgInBzIjogIlNPTUVWNiIsDQogICJhZGQiOiAiaXB2Ni5zZy53aG8ubW9lIiwNCiAgInBvcnQiOiAiNDA0IiwNCiAgImlkIjogIjY4YmI2MGUyLTJhMDQtNDk2Yy1iYjM4LWE0NDkyMDZhNGJkZSIsDQogICJhaWQiOiAiNjQiLA0KICAibmV0IjogIndzIiwNCiAgInR5cGUiOiAibm9uZSIsDQogICJob3N0IjogImlwdjYuc2cud2hvLm1vZSIsDQogICJwYXRoIjogIi93aGVyZS8iLA0KICAidGxzIjogInRscyINCn0="#;
        let ous = r#"vmess://U09NRVY2ID0gdm1lc3MsIGlwdjYuc2cud2hvLm1vZSwgNDA0LCBjaGFjaGEyMC1pZXRmLXBvbHkxMzA1LCAiNjhiYjYwZTItMmEwNC00OTZjLWJiMzgtYTQ0OTIwNmE0YmRlIiwgZ3JvdXA9VjJOdG9RdWFuLCBvdmVyLXRscz10cnVlLCBjZXJ0aWZpY2F0ZT0xLCBvYmZzPXdzLCBvYmZzLXBhdGg9Ii93aGVyZS8iLCBvYmZzLWhlYWRlcj0iSG9zdDogaXB2Ni5zZy53aG8ubW9lW1JyXVtObl1Vc2VyLUFnZW50OiBNb3ppbGxhLzUuMCAoaVBob25lOyBDUFUgaVBob25lIE9TIDExXzJfNiBsaWtlIE1hYyBPUyBYKSBBcHBsZVdlYktpdC82MDQuNS42IChLSFRNTCwgbGlrZSBHZWNrbykgTW9iaWxlLzE1RDEwMCI="#;
        check_convert(&ins, &ous);
    }
}
