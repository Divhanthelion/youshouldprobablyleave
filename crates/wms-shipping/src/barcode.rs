//! Barcode Scanning and Decoding
//! 
//! Uses rxing (Rust port of ZXing) for decoding barcodes from images.

use rxing::{BarcodeFormat, DecodeHintType, DecodeHintValue, DecodingHintDictionary, Luma8LuminanceSource, Reader};
use rxing::multi::GenericMultipleBarcodeReader;
use rxing::oned::MultiFormatOneDReader;
use rxing::common::HybridBinarizer;
use rxing::BinaryBitmap;
use serde::{Deserialize, Serialize};
use wms_core::error::{WmsError, Result};

/// Barcode decoder using rxing
pub struct BarcodeDecoder {
    hints: DecodingHintDictionary,
}

impl BarcodeDecoder {
    /// Create a new barcode decoder
    pub fn new() -> Self {
        let mut hints = DecodingHintDictionary::new();
        
        // Enable multiple barcode formats
        hints.insert(
            DecodeHintType::POSSIBLE_FORMATS,
            DecodeHintValue::PossibleFormats(vec![
                BarcodeFormat::EAN_13,
                BarcodeFormat::EAN_8,
                BarcodeFormat::UPC_A,
                BarcodeFormat::UPC_E,
                BarcodeFormat::CODE_128,
                BarcodeFormat::CODE_39,
                BarcodeFormat::QR_CODE,
                BarcodeFormat::PDF_417,
                BarcodeFormat::DATA_MATRIX,
            ].into_iter().collect()),
        );
        
        // Try harder to find barcodes
        hints.insert(DecodeHintType::TRY_HARDER, DecodeHintValue::TryHarder(true));
        
        Self { hints }
    }
    
    /// Decode a barcode from grayscale image data
    pub fn decode(&self, image_data: &[u8], width: u32, height: u32) -> Result<BarcodeResult> {
        // Create luminance source from raw grayscale data
        let source = Luma8LuminanceSource::new(
            image_data.to_vec(),
            width,
            height,
        );
        
        // Create binary bitmap
        let binarizer = HybridBinarizer::new(source);
        let bitmap = BinaryBitmap::new(binarizer);
        
        // Try decoding with multi-format reader
        let reader = MultiFormatOneDReader::default();
        
        match reader.decode_with_hints(&bitmap, &self.hints) {
            Ok(result) => {
                Ok(BarcodeResult {
                    text: result.getText().to_string(),
                    format: Self::format_to_string(result.getBarcodeFormat()),
                    raw_bytes: result.getRawBytes().map(|b| b.to_vec()),
                    orientation: Some(0), // Could extract from ResultPoint
                    confidence: 1.0, // rxing doesn't provide confidence
                })
            }
            Err(e) => {
                Err(WmsError::Barcode(format!("Failed to decode barcode: {:?}", e)))
            }
        }
    }
    
    /// Decode multiple barcodes from an image
    pub fn decode_multiple(&self, image_data: &[u8], width: u32, height: u32) -> Result<Vec<BarcodeResult>> {
        let source = Luma8LuminanceSource::new(
            image_data.to_vec(),
            width,
            height,
        );
        
        let binarizer = HybridBinarizer::new(source);
        let bitmap = BinaryBitmap::new(binarizer);
        
        let reader = MultiFormatOneDReader::default();
        let multi_reader = GenericMultipleBarcodeReader::new(reader);
        
        match multi_reader.decode_multiple_with_hints(&bitmap, &self.hints) {
            Ok(results) => {
                Ok(results.into_iter().map(|r| BarcodeResult {
                    text: r.getText().to_string(),
                    format: Self::format_to_string(r.getBarcodeFormat()),
                    raw_bytes: r.getRawBytes().map(|b| b.to_vec()),
                    orientation: None,
                    confidence: 1.0,
                }).collect())
            }
            Err(e) => {
                Err(WmsError::Barcode(format!("Failed to decode barcodes: {:?}", e)))
            }
        }
    }
    
    /// Generate a barcode image
    pub fn generate(&self, text: &str, format: BarcodeFormat, width: u32, height: u32) -> Result<Vec<u8>> {
        use rxing::Writer;
        use rxing::oned::Code128Writer;
        use rxing::qrcode::QRCodeWriter;
        
        let result = match format {
            BarcodeFormat::CODE_128 => {
                let writer = Code128Writer;
                writer.encode(text, &format, width as i32, height as i32)
            }
            BarcodeFormat::QR_CODE => {
                let writer = QRCodeWriter;
                writer.encode(text, &format, width as i32, height as i32)
            }
            _ => {
                return Err(WmsError::Barcode(format!("Unsupported format for generation: {:?}", format)));
            }
        };
        
        match result {
            Ok(matrix) => {
                // Convert BitMatrix to grayscale image data
                let mut image_data = vec![255u8; (width * height) as usize];
                for y in 0..height as usize {
                    for x in 0..width as usize {
                        if matrix.get(x as u32, y as u32) {
                            image_data[y * width as usize + x] = 0;
                        }
                    }
                }
                Ok(image_data)
            }
            Err(e) => {
                Err(WmsError::Barcode(format!("Failed to generate barcode: {:?}", e)))
            }
        }
    }
    
    fn format_to_string(format: &BarcodeFormat) -> String {
        match format {
            BarcodeFormat::EAN_13 => "EAN-13",
            BarcodeFormat::EAN_8 => "EAN-8",
            BarcodeFormat::UPC_A => "UPC-A",
            BarcodeFormat::UPC_E => "UPC-E",
            BarcodeFormat::CODE_128 => "CODE-128",
            BarcodeFormat::CODE_39 => "CODE-39",
            BarcodeFormat::QR_CODE => "QR",
            BarcodeFormat::PDF_417 => "PDF417",
            BarcodeFormat::DATA_MATRIX => "DATA-MATRIX",
            _ => "UNKNOWN",
        }.to_string()
    }
}

impl Default for BarcodeDecoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of barcode decoding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarcodeResult {
    /// Decoded text content
    pub text: String,
    /// Barcode format (EAN-13, QR, etc.)
    pub format: String,
    /// Raw bytes (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_bytes: Option<Vec<u8>>,
    /// Orientation in degrees
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<i32>,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
}

impl BarcodeResult {
    /// Check if this is a product barcode (EAN/UPC)
    pub fn is_product_barcode(&self) -> bool {
        matches!(
            self.format.as_str(),
            "EAN-13" | "EAN-8" | "UPC-A" | "UPC-E"
        )
    }
    
    /// Check if this is a shipping barcode (CODE-128)
    pub fn is_shipping_barcode(&self) -> bool {
        matches!(self.format.as_str(), "CODE-128" | "CODE-39")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_decoder_creation() {
        let decoder = BarcodeDecoder::new();
        assert!(decoder.hints.len() > 0);
    }
}

