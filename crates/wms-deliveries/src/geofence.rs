//! Geofencing Module
//! 
//! Provides geospatial analysis for delivery zone detection using the geo crate.

use geo::{Contains, Point, Polygon, LineString, coord};
use serde::{Deserialize, Serialize};
use crate::models::GeoPoint;

/// Result of a geofence check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeofenceResult {
    /// Whether the point is inside the geofence
    pub is_inside: bool,
    /// Distance to the geofence boundary in meters
    pub distance_to_boundary_meters: f64,
    /// Geofence name/ID that was checked
    pub geofence_id: Option<String>,
    /// Trigger type if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<GeofenceTrigger>,
}

/// Geofence trigger types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GeofenceTrigger {
    Enter,
    Exit,
    Dwell,
}

/// Geofence geometry types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GeofenceGeometry {
    Circle {
        center: GeoPoint,
        radius_meters: f64,
    },
    Polygon {
        vertices: Vec<GeoPoint>,
    },
}

/// Geofence checker for spatial analysis
pub struct GeofenceChecker {
    /// Track previous states for enter/exit detection
    previous_states: std::collections::HashMap<String, bool>,
}

impl GeofenceChecker {
    /// Create a new geofence checker
    pub fn new() -> Self {
        Self {
            previous_states: std::collections::HashMap::new(),
        }
    }
    
    /// Check if a point is inside a circular geofence
    pub fn check_circle(
        &self,
        point: GeoPoint,
        center: GeoPoint,
        radius_meters: f64,
    ) -> GeofenceResult {
        let distance = Self::haversine_distance(point, center);
        
        GeofenceResult {
            is_inside: distance <= radius_meters,
            distance_to_boundary_meters: (distance - radius_meters).abs(),
            geofence_id: None,
            trigger_type: None,
        }
    }
    
    /// Check if a point is inside a polygon geofence
    pub fn check_polygon(
        &self,
        point: GeoPoint,
        vertices: &[GeoPoint],
    ) -> GeofenceResult {
        if vertices.len() < 3 {
            return GeofenceResult {
                is_inside: false,
                distance_to_boundary_meters: f64::MAX,
                geofence_id: None,
                trigger_type: None,
            };
        }
        
        // Convert to geo types
        let geo_point = Point::new(point.lng, point.lat);
        
        let coords: Vec<_> = vertices.iter()
            .map(|v| coord! { x: v.lng, y: v.lat })
            .collect();
        
        let line_string = LineString::new(coords);
        let polygon = Polygon::new(line_string, vec![]);
        
        let is_inside = polygon.contains(&geo_point);
        
        // Calculate distance to boundary (simplified)
        let distance = if is_inside {
            Self::distance_to_polygon_boundary(&geo_point, &polygon)
        } else {
            Self::distance_to_polygon_boundary(&geo_point, &polygon)
        };
        
        GeofenceResult {
            is_inside,
            distance_to_boundary_meters: distance,
            geofence_id: None,
            trigger_type: None,
        }
    }
    
    /// Check geofence with enter/exit tracking
    pub fn check_with_tracking(
        &mut self,
        geofence_id: &str,
        point: GeoPoint,
        geometry: &GeofenceGeometry,
    ) -> GeofenceResult {
        let is_inside = match geometry {
            GeofenceGeometry::Circle { center, radius_meters } => {
                Self::haversine_distance(point, *center) <= *radius_meters
            }
            GeofenceGeometry::Polygon { vertices } => {
                self.check_polygon(point, vertices).is_inside
            }
        };
        
        let previous = self.previous_states.get(geofence_id).copied().unwrap_or(false);
        let trigger = match (previous, is_inside) {
            (false, true) => Some(GeofenceTrigger::Enter),
            (true, false) => Some(GeofenceTrigger::Exit),
            _ => None,
        };
        
        self.previous_states.insert(geofence_id.to_string(), is_inside);
        
        let distance = match geometry {
            GeofenceGeometry::Circle { center, radius_meters } => {
                (Self::haversine_distance(point, *center) - radius_meters).abs()
            }
            GeofenceGeometry::Polygon { .. } => 0.0, // Simplified
        };
        
        GeofenceResult {
            is_inside,
            distance_to_boundary_meters: distance,
            geofence_id: Some(geofence_id.to_string()),
            trigger_type: trigger,
        }
    }
    
    /// Calculate Haversine distance in meters
    fn haversine_distance(p1: GeoPoint, p2: GeoPoint) -> f64 {
        const EARTH_RADIUS_M: f64 = 6_371_000.0;
        
        let lat1 = p1.lat.to_radians();
        let lat2 = p2.lat.to_radians();
        let dlat = (p2.lat - p1.lat).to_radians();
        let dlng = (p2.lng - p1.lng).to_radians();
        
        let a = (dlat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (dlng / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().asin();
        
        EARTH_RADIUS_M * c
    }
    
    /// Calculate distance from point to polygon boundary (simplified)
    fn distance_to_polygon_boundary(point: &Point<f64>, polygon: &Polygon<f64>) -> f64 {
        // Simplified: find minimum distance to any vertex
        let exterior = polygon.exterior();
        let mut min_dist = f64::MAX;
        
        for coord in exterior.coords() {
            let vertex = Point::new(coord.x, coord.y);
            let dist = Self::haversine_distance(
                GeoPoint::new(point.y(), point.x()),
                GeoPoint::new(vertex.y(), vertex.x()),
            );
            if dist < min_dist {
                min_dist = dist;
            }
        }
        
        min_dist
    }
}

impl Default for GeofenceChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_circle_geofence_inside() {
        let checker = GeofenceChecker::new();
        
        let center = GeoPoint::new(40.7128, -74.0060);
        let point = GeoPoint::new(40.7130, -74.0062); // Very close
        
        let result = checker.check_circle(point, center, 100.0);
        assert!(result.is_inside);
    }
    
    #[test]
    fn test_circle_geofence_outside() {
        let checker = GeofenceChecker::new();
        
        let center = GeoPoint::new(40.7128, -74.0060);
        let point = GeoPoint::new(40.7200, -74.0100); // ~1km away
        
        let result = checker.check_circle(point, center, 100.0);
        assert!(!result.is_inside);
    }
    
    #[test]
    fn test_polygon_geofence() {
        let checker = GeofenceChecker::new();
        
        let vertices = vec![
            GeoPoint::new(40.0, -74.0),
            GeoPoint::new(40.0, -73.0),
            GeoPoint::new(41.0, -73.0),
            GeoPoint::new(41.0, -74.0),
        ];
        
        let inside = GeoPoint::new(40.5, -73.5);
        let outside = GeoPoint::new(42.0, -72.0);
        
        assert!(checker.check_polygon(inside, &vertices).is_inside);
        assert!(!checker.check_polygon(outside, &vertices).is_inside);
    }
    
    #[test]
    fn test_enter_exit_tracking() {
        let mut checker = GeofenceChecker::new();
        
        let geofence = GeofenceGeometry::Circle {
            center: GeoPoint::new(40.7128, -74.0060),
            radius_meters: 100.0,
        };
        
        // Start outside
        let outside = GeoPoint::new(40.7200, -74.0100);
        let result1 = checker.check_with_tracking("test", outside, &geofence);
        assert!(!result1.is_inside);
        assert!(result1.trigger_type.is_none());
        
        // Enter
        let inside = GeoPoint::new(40.7129, -74.0061);
        let result2 = checker.check_with_tracking("test", inside, &geofence);
        assert!(result2.is_inside);
        assert_eq!(result2.trigger_type, Some(GeofenceTrigger::Enter));
        
        // Exit
        let result3 = checker.check_with_tracking("test", outside, &geofence);
        assert!(!result3.is_inside);
        assert_eq!(result3.trigger_type, Some(GeofenceTrigger::Exit));
    }
}

