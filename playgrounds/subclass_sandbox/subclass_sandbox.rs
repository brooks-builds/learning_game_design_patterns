trait SuperPower {
    fn activate(&mut self, hero: &mut Hero, sound_system: &mut SoundSystem);

    fn move_hero(&mut self, x: f32, y: f32, z: f32, hero: &mut Hero) {
        // move the hero
        hero.apply_force([x, y, z]);
    }

    fn play_sound(&self, soundId: i32, volume: f32, sound_system: &mut SoundSystem) {
        // play this sound by default
        sound_system.playsound(soundId, volume);
    }
}

struct SkyLaunch {}

impl SuperPower for SkyLaunch {
    fn activate(&mut self, hero: &mut Hero, sound_system: &mut SoundSystem) {
        // launch into the sky
        self.moveHero(0.0, 20.0, 0.0, &mut hero);
        self.play_sound(JUMP_SOUND, 1.0, &mut sound_system);
    }
}