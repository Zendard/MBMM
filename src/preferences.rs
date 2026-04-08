use field_count::FieldCount;
use std::{error::Error, path::PathBuf};

#[derive(FieldCount, Clone, Debug)]
pub struct Preferences {
    game_dir: PathBuf,
}

impl Preferences {
    pub fn load() -> Option<Self> {
        let mut file_path = dirs::config_dir()?;
        file_path.push(PathBuf::from("MBMM"));
        file_path.push(PathBuf::from("preferences"));

        let file = std::fs::read_to_string(file_path).ok()?;
        Preferences::decode(&file).ok()
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let string = self.encode()?;
        let mut file_path = dirs::config_dir().ok_or("Can't get config_dir")?;

        file_path.push(PathBuf::from("MBMM"));
        // Make config dir if it doesn't exist
        if !file_path.try_exists().unwrap_or(false) {
            std::fs::create_dir(&file_path)?
        }

        file_path.push(PathBuf::from("MBMM"));

        std::fs::write(file_path, string)?;
        Ok(())
    }

    fn encode(&self) -> Result<String, Box<dyn Error>> {
        let mut lines = Vec::with_capacity(Self::field_count());
        lines.push(
            self.game_dir
                .to_str()
                .ok_or("Can't transform game_dir into string")?,
        );

        Ok(lines.join("\n"))
    }

    fn decode(file: &str) -> Result<Self, Box<dyn Error>> {
        // Config file format:
        // game_dir
        let mut lines = file.lines();
        let game_dir = lines.next().ok_or("No game_dir value")?.try_into()?;
        Ok(Self { game_dir })
    }
}
