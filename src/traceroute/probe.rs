use std::fmt::Display;
use std::io;
use std::net::Ipv4Addr;
use std::time::{Duration, Instant};

pub use parser::ProbeResponseParser;

pub mod parser;
pub mod task;
pub mod generator;
pub mod sniffer;

pub type ProbeId = String;

#[derive(Clone, Debug)]
pub struct ProbeResponse {
    id: ProbeId,
    from_address: Ipv4Addr,
}

impl ProbeResponse {
    pub fn probe_id(&self) -> ProbeId {
        self.id.clone()
    }
    
    pub fn from_address(&self) -> Ipv4Addr {
        self.from_address
    }
}

#[derive(Clone, Debug)]
pub struct ProbeResult {
    id: ProbeId,
    ttl: u8,
    from_address: Ipv4Addr,
    rtt: Duration,
    hostname: Option<String>,
}

impl ProbeResult {
    pub fn probe_id(&self) -> ProbeId {
        self.id.clone()
    }

    pub fn from_address(&self) -> Ipv4Addr {
        self.from_address
    }
    
    pub fn rtt(&self) -> Duration {
        self.rtt
    }
    
    pub fn ttl(&self) -> u8 {
        self.ttl
    }
    
    pub fn set_hostname(&mut self, hostname: &str) {
        self.hostname = Some(hostname.to_string());
    }
    pub fn get_hostname(&self) -> Option<String> {
        self.hostname.clone()
    }
}

impl Display for ProbeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::with_capacity(300);
        let from_address = self.from_address;
        string.push_str(&self.hostname.as_ref().unwrap_or(&from_address.to_string()));
        string.push(' ');
        string.push_str(&format!("({from_address})"));
        string.push_str("  ");
        
        let mut rtt_micros = self.rtt.as_micros().to_string();
        rtt_micros.insert(2, '.');
        string.push_str(&format!("{:2} ms", rtt_micros));
        write!(f, "{}", string)
    }
}

#[derive(Debug)]
pub enum ProbeError {
    Timeout { ttl: u8 },
    IoError { ttl: u8, io_error: Option<io::Error> },
}

impl ProbeError {
    pub fn get_ttl(&self) -> u8 {
        match self {
            ProbeError::Timeout { ttl } => *ttl,
            ProbeError::IoError { ttl, .. } => *ttl
        }
    }
}

struct CompletableProbe {
    id: ProbeId,
    ttl: u8,
    sent_at: Instant,
    probe_result: Option<ProbeResult>,
}

impl CompletableProbe {
    pub fn new(id: &str, ttl: u8) -> Self {
        Self {
            id: id.to_string(),
            ttl,
            sent_at: Instant::now(),
            probe_result: None,
        }
    }

    pub fn complete(&mut self, probe_response: ProbeResponse) -> Option<ProbeResult> {
        if probe_response.id != self.id {
            return None;
        }

        if let Some(probe_result) = &self.probe_result {
            return Some(probe_result.clone());
        }

        Some(ProbeResult {
            id: probe_response.id,
            ttl: self.ttl,
            from_address: probe_response.from_address,
            rtt: self.sent_at.elapsed(),
            hostname: None,
        })
    }
}

#[derive(Debug, clap::ValueEnum, Clone, Default)]
pub enum ProbeMethod {
    #[default]
    UDP,
    TCP,
    ICMP,
}