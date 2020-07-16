const MAX_VOLUME: f32 = 1.0;
const MAX_AUDIO_PENDING: usize = 16;

#[derive(Copy, Clone, PartialEq)]
enum Sound {
    Swoop,
}

#[derive(Debug)]
enum CustomError {
    AudioQueueOverflow,
    AudioSoundNotFound,
}

#[derive(Copy, Clone)]
struct PlayMessage {
    id: Option<Sound>,
    volume: Option<f32>,
}

impl PlayMessage {
    pub fn new() -> PlayMessage {
        PlayMessage {
            id: None,
            volume: None,
        }
    }
}

struct Audio {
    head: usize,
    tail: usize,
    pending: [PlayMessage; MAX_AUDIO_PENDING],
}

impl Audio {
    pub fn new() -> Audio {
        Audio {
            head: 0,
            tail: 0,
            pending: [PlayMessage::new(); MAX_AUDIO_PENDING],
        }
    }

    pub fn play_sound(&mut self, id: Sound, volume: f32) -> Result<(), CustomError> {
        let mut index = self.head;
        while index != self.tail {
            let pending_id = if let Some(id) = self.pending[index].id {
                id
            } else {
                continue;
            };
            if id == pending_id {
                self.pending[index].volume = Some(volume);
                return Ok(());
            }
            index = (index + 1) % MAX_AUDIO_PENDING;
        }
        if (self.tail + 1) % MAX_AUDIO_PENDING == self.head {
            return Err(CustomError::AudioQueueOverflow);
        }

        self.pending[self.tail].id = Some(id);
        self.pending[self.tail].volume = Some(volume);
        self.tail = (self.tail + 1) % MAX_AUDIO_PENDING;
        Ok(())
    }
    pub fn update(&mut self) -> Result<(), CustomError> {
        if self.head == self.tail {
            return Ok(());
        }

        let id = if let Some(id) = self.pending[self.head].id {
            id
        } else {
            return Err(CustomError::AudioSoundNotFound);
        };
        let volume = if let Some(volume) = self.pending[self.head].volume {
            volume
        } else {
            return Err(CustomError::AudioSoundNotFound);
        };
        let resource = load_sound(id);
        let channel = if let Some(inner_channel) = find_open_channel() {
            inner_channel
        } else {
            return Ok(());
        };
        start_sound(resource, channel, volume);
        self.head = (self.head + 1) % MAX_AUDIO_PENDING;
        Ok(())
    }
}

fn main() -> Result<(), CustomError> {
    let mut audio = Audio::new();
    audio.play_sound(Sound::Swoop, 0.5)?;
    audio.play_sound(Sound::Swoop, MAX_VOLUME)?;
    audio.update()?;
    audio.update()?;

    Ok(())
}

fn load_sound(id: Sound) -> String {
    match id {
        Sound::Swoop => String::from("swoop sounds"),
    }
}

fn find_open_channel() -> Option<i32> {
    Some(4)
}

fn start_sound(resource: String, channel: i32, volume: f32) {
    println!(
        "Playing sound: {} on channel {} with volume {}",
        resource, channel, volume
    );
}
