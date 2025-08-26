use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub enum Section {
    #[default]
    Currently,
    Later,
    Archive,
    Custom(String),
}

impl Section {
    pub fn parse(s: &str) -> Self {
        let trimmed = s.to_lowercase();
        let trimmed = trimmed.trim_end_matches(':');
        match trimmed {
            "currently" | "current" => Section::Currently,
            "later" => Section::Later,
            "archive" | "archived" => Section::Archive,
            _ => Section::Custom(s.trim_end_matches(':').to_string()),
        }
    }

    pub fn to_taskpaper(&self) -> String {
        match self {
            Section::Currently => "Currently:".to_string(),
            Section::Later => "Later:".to_string(),
            Section::Archive => "Archive:".to_string(),
            Section::Custom(name) => format!("{name}:"),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Section::Currently => "Currently",
            Section::Later => "Later",
            Section::Archive => "Archive",
            Section::Custom(name) => name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_from_str() {
        assert_eq!(Section::parse("Currently:"), Section::Currently);
        assert_eq!(Section::parse("later"), Section::Later);
        assert_eq!(Section::parse("Archive:"), Section::Archive);
        assert_eq!(Section::parse("Work:"), Section::Custom("Work".to_string()));
    }

    #[test]
    fn test_section_to_taskpaper() {
        assert_eq!(Section::Currently.to_taskpaper(), "Currently:");
        assert_eq!(Section::Later.to_taskpaper(), "Later:");
        assert_eq!(
            Section::Custom("Project".to_string()).to_taskpaper(),
            "Project:"
        );
    }
}
