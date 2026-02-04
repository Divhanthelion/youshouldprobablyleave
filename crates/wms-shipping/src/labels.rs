//! Label Generation
//! 
//! ZPL (Zebra Programming Language) label generation for thermal printers
//! and PDF generation for standard documents.

use printpdf::*;
use std::io::BufWriter;

/// ZPL Label Builder
/// 
/// Generates ZPL code for Zebra thermal printers.
#[derive(Debug, Clone)]
pub struct ZplLabel {
    elements: Vec<String>,
    width_dots: u32,
    height_dots: u32,
    dpi: u32,
}

impl ZplLabel {
    /// Create a new ZPL label (default 203 DPI)
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            width_dots: 812,  // 4 inches at 203 DPI
            height_dots: 1218, // 6 inches at 203 DPI
            dpi: 203,
        }
    }
    
    /// Set label size in inches
    pub fn set_size(mut self, width_inches: u32, height_inches: u32) -> Self {
        self.width_dots = width_inches * self.dpi;
        self.height_dots = height_inches * self.dpi;
        self
    }
    
    /// Set DPI (203, 300, or 600)
    pub fn set_dpi(mut self, dpi: u32) -> Self {
        self.dpi = dpi;
        self
    }
    
    /// Add text to the label
    pub fn add_text(mut self, x: u32, y: u32, text: &str, font: char, height: u32) -> Self {
        // ^FO = Field Origin, ^A = Font, ^FD = Field Data, ^FS = Field Separator
        self.elements.push(format!(
            "^FO{},{}^A{},{}^FD{}^FS",
            x, y, font, height, Self::escape_text(text)
        ));
        self
    }
    
    /// Add a Code 128 barcode
    pub fn add_barcode_128(mut self, x: u32, y: u32, data: &str, height: u32) -> Self {
        // ^BC = Code 128 barcode
        self.elements.push(format!(
            "^FO{},{}^BCN,{},Y,N,N^FD{}^FS",
            x, y, height, data
        ));
        self
    }
    
    /// Add a Code 39 barcode
    pub fn add_barcode_39(mut self, x: u32, y: u32, data: &str, height: u32) -> Self {
        // ^B3 = Code 39 barcode
        self.elements.push(format!(
            "^FO{},{}^B3N,N,{},Y,N^FD{}^FS",
            x, y, height, data
        ));
        self
    }
    
    /// Add a QR code
    pub fn add_qr_code(mut self, x: u32, y: u32, data: &str, magnification: u32) -> Self {
        // ^BQ = QR Code
        self.elements.push(format!(
            "^FO{},{}^BQN,2,{}^FDQA,{}^FS",
            x, y, magnification, data
        ));
        self
    }
    
    /// Add a horizontal line
    pub fn add_line(mut self, x: u32, y: u32, width: u32, thickness: u32) -> Self {
        // ^GB = Graphic Box
        self.elements.push(format!(
            "^FO{},{}^GB{},{},{}^FS",
            x, y, width, thickness, thickness
        ));
        self
    }
    
    /// Add a box/rectangle
    pub fn add_box(mut self, x: u32, y: u32, width: u32, height: u32, border: u32) -> Self {
        self.elements.push(format!(
            "^FO{},{}^GB{},{},{}^FS",
            x, y, width, height, border
        ));
        self
    }
    
    /// Add a graphic (raw binary data)
    pub fn add_graphic(mut self, x: u32, y: u32, width: u32, height: u32, data: &[u8]) -> Self {
        let total_bytes = data.len();
        let hex_data: String = data.iter().map(|b| format!("{:02X}", b)).collect();
        
        // ^GF = Graphic Field
        self.elements.push(format!(
            "^FO{},{}^GFA,{},{},{},{}^FS",
            x, y, total_bytes, total_bytes, width / 8, hex_data
        ));
        self
    }
    
    /// Build the final ZPL string
    pub fn build(&self) -> String {
        let mut zpl = String::new();
        
        // Start format
        zpl.push_str("^XA\n");
        
        // Set label dimensions
        zpl.push_str(&format!("^PW{}\n", self.width_dots));
        zpl.push_str(&format!("^LL{}\n", self.height_dots));
        
        // Add all elements
        for element in &self.elements {
            zpl.push_str(element);
            zpl.push('\n');
        }
        
        // End format and print
        zpl.push_str("^XZ\n");
        
        zpl
    }
    
    /// Escape special characters in text
    fn escape_text(text: &str) -> String {
        text.replace('\\', "\\\\")
            .replace('^', "\\^")
            .replace('~', "\\~")
    }
}

impl Default for ZplLabel {
    fn default() -> Self {
        Self::new()
    }
}

/// PDF Generator for shipping documents
pub struct PdfGenerator {
    doc: PdfDocumentReference,
    current_page: PdfPageIndex,
    current_layer: PdfLayerIndex,
}

impl PdfGenerator {
    /// Create a new PDF document
    pub fn new(title: &str) -> Self {
        let (doc, page1, layer1) = PdfDocument::new(
            title,
            Mm(210.0), // A4 width
            Mm(297.0), // A4 height
            "Layer 1",
        );

        Self {
            doc,
            current_page: page1,
            current_layer: layer1,
        }
    }

    /// Create a shipping document (letter size)
    pub fn new_shipping_doc(title: &str) -> Self {
        let (doc, page1, layer1) = PdfDocument::new(
            title,
            Mm(215.9), // Letter width
            Mm(279.4), // Letter height
            "Layer 1",
        );

        Self {
            doc,
            current_page: page1,
            current_layer: layer1,
        }
    }

    /// Add a new page
    pub fn add_page(&mut self) {
        let (page, layer) = self.doc.add_page(
            Mm(215.9),
            Mm(279.4),
            "Layer 1",
        );
        self.current_page = page;
        self.current_layer = layer;
    }

    /// Add text to the current page
    pub fn add_text(&self, x: f32, y: f32, text: &str, font_size: f32) {
        let font = self.doc.add_builtin_font(BuiltinFont::Helvetica).unwrap();
        let current_layer = self.doc.get_page(self.current_page).get_layer(self.current_layer);

        current_layer.use_text(text, font_size, Mm(x), Mm(y), &font);
    }

    /// Add bold text
    pub fn add_bold_text(&self, x: f32, y: f32, text: &str, font_size: f32) {
        let font = self.doc.add_builtin_font(BuiltinFont::HelveticaBold).unwrap();
        let current_layer = self.doc.get_page(self.current_page).get_layer(self.current_layer);

        current_layer.use_text(text, font_size, Mm(x), Mm(y), &font);
    }

    /// Draw a line
    pub fn draw_line(&self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let current_layer = self.doc.get_page(self.current_page).get_layer(self.current_layer);

        let points = vec![
            (Point::new(Mm(x1), Mm(y1)), false),
            (Point::new(Mm(x2), Mm(y2)), false),
        ];

        let line = Line {
            points,
            is_closed: false,
        };

        current_layer.add_line(line);
    }

    /// Draw a rectangle
    pub fn draw_rect(&self, x: f32, y: f32, width: f32, height: f32) {
        let current_layer = self.doc.get_page(self.current_page).get_layer(self.current_layer);

        let points = vec![
            (Point::new(Mm(x), Mm(y)), false),
            (Point::new(Mm(x + width), Mm(y)), false),
            (Point::new(Mm(x + width), Mm(y + height)), false),
            (Point::new(Mm(x), Mm(y + height)), false),
        ];

        let rect = Line {
            points,
            is_closed: true,
        };

        current_layer.add_line(rect);
    }

    /// Save to bytes
    pub fn save_to_bytes(self) -> Vec<u8> {
        let mut buffer = BufWriter::new(Vec::new());
        self.doc.save(&mut buffer).unwrap();
        buffer.into_inner().unwrap()
    }
}

/// Create a packing slip PDF
pub fn create_packing_slip(
    shipment_number: &str,
    ship_to: &str,
    items: &[(String, String, f64)], // (SKU, Name, Qty)
) -> Vec<u8> {
    let pdf = PdfGenerator::new_shipping_doc("Packing Slip");
    
    // Header
    pdf.add_bold_text(20.0, 270.0, "PACKING SLIP", 18.0);
    pdf.add_text(20.0, 260.0, &format!("Shipment: {}", shipment_number), 12.0);
    
    // Ship to
    pdf.add_bold_text(20.0, 240.0, "Ship To:", 12.0);
    let mut y = 230.0;
    for line in ship_to.lines() {
        pdf.add_text(20.0, y, line, 10.0);
        y -= 5.0;
    }
    
    // Items table header
    pdf.draw_line(20.0, 200.0, 195.0, 200.0);
    pdf.add_bold_text(20.0, 195.0, "SKU", 10.0);
    pdf.add_bold_text(60.0, 195.0, "Description", 10.0);
    pdf.add_bold_text(170.0, 195.0, "Qty", 10.0);
    pdf.draw_line(20.0, 190.0, 195.0, 190.0);
    
    // Items
    let mut y = 185.0;
    for (sku, name, qty) in items {
        pdf.add_text(20.0, y, sku, 9.0);
        pdf.add_text(60.0, y, name, 9.0);
        pdf.add_text(170.0, y, &format!("{:.0}", qty), 9.0);
        y -= 6.0;
    }
    
    pdf.save_to_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_zpl_label() {
        let zpl = ZplLabel::new()
            .set_size(4, 6)
            .add_text(50, 50, "Test Label", 'A', 40)
            .add_barcode_128(50, 150, "123456789", 80)
            .build();
        
        assert!(zpl.contains("^XA"));
        assert!(zpl.contains("^XZ"));
        assert!(zpl.contains("Test Label"));
        assert!(zpl.contains("123456789"));
    }
    
    #[test]
    fn test_zpl_qr_code() {
        let zpl = ZplLabel::new()
            .add_qr_code(100, 100, "https://example.com", 5)
            .build();
        
        assert!(zpl.contains("^BQ"));
        assert!(zpl.contains("https://example.com"));
    }
}

