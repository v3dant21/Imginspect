use crate::format::ImageFormat;
use std::fmt;

#[derive(Debug)]
pub struct MetadataItem {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub struct ImageReport {
    pub format: ImageFormat,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub metadata: Vec<MetadataItem>,
    pub binary_info: Vec<String>,
}

impl fmt::Display for ImageReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Format: {}", self.format)?;

        if let (Some(w), Some(h)) = (self.width, self.height) {
            writeln!(f, "Dimensions: {} x {}", w, h)?;
        }

        if !self.metadata.is_empty() {
            writeln!(f, "\nMetadata:")?;
            for m in &self.metadata {
                writeln!(f, "  {}: {}", m.key, m.value)?;
            }
        }

        if !self.binary_info.is_empty() {
            writeln!(f, "\nBinary Info:")?;
            for b in &self.binary_info {
                writeln!(f, "  {}", b)?;
            }
        }

        Ok(())
    }
}
impl ImageReport {
    // New method to print only high-level metadata
    pub fn display_metadata_only(&self) {
        println!("Format: {}", self.format);
        if let (Some(w), Some(h)) = (self.width, self.height) {
            println!("Dimensions: {} x {}", w, h);
        }
        if !self.metadata.is_empty() {
            println!("\nMetadata:");
            for m in &self.metadata {
                println!("  {}: {}", m.key, m.value);
            }
        }
    }
}
