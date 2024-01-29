#[derive(Clone)]
pub struct Daemon
{
    // daemon_ptr: Option<Daemon&>
    signals: Vec<Signal>
}

// TODO make signal execution
impl Daemon
{
    pub fn new() -> Self {
        Daemon {
            signals: Vec::new()
        }
    }

    pub fn send_signal(self: &mut Self, s: &Signal) {
        self.signals.push(s.clone());
    }
}

#[derive(Clone)]
pub struct Signal
{
    pub name: String,
    pub detail: String,
    pub status_code: u8
}

impl Signal
{
    pub fn from(n: &String, d: &String, c: &u8) -> Self
    {
        Signal {
            name: n.clone(),
            detail: d.clone(),
            status_code: c.clone()
        }
    }
}
