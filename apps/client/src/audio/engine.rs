use crossbeam_channel::Receiver;

#[derive(Debug)]
#[non_exhaustive]
pub enum AudioCommand {
    StartStreaming(String), // e.g. UDP IP:Port
    Mute(bool),
    Shutdown,
    ChangeInputDevice(u32),
    ChangeOutputDevice(u32),
}

#[derive(Debug)]
pub struct AudioEngine {
    pub command_rx: Receiver<AudioCommand>,
    // std::net::UdpSocket goes here (synchronus / blocking);
}

impl AudioEngine {
    pub fn new(command_rx: Receiver<AudioCommand>) -> Self {
        AudioEngine {
            command_rx
        }
    }
    pub fn run (mut self) {
        // Initialize Audio Hardware (cpal)
        // Bind std::net::UdpSocket

        loop {
            // 1. Check for UI/Control commands (non-blocking)
            if let Ok(cmd) = self.command_rx.try_recv() {
                match cmd {
                    AudioCommand::Mute(state) => println!("Hardware Mute: {}", state),
                    AudioCommand::Shutdown => break,
                    any => { tracing::debug!("Unhandled AudioCommand: {:?}", any)}
                }
            }
        }

        // 2. Handle UDP / Audio mixing (blocking or tight loop)
        // Read from UDP -> Decode -> Mix -> Audio Buffer
        // Read Audio Buffer -> Encode -> Send UDP
        // NOTE: Keep allocations and locks out of this loop!
    }
}


