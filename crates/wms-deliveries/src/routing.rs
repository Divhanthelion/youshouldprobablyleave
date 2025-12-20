//! Route Optimization
//! 
//! Vehicle Routing Problem (VRP) solver using heuristic algorithms.
//! Runs entirely on-device for offline capability.

use serde::{Deserialize, Serialize};
use wms_core::error::{WmsError, Result};
use crate::models::GeoPoint;

/// Optimized route result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedRoute {
    /// Ordered list of delivery IDs
    pub stop_order: Vec<String>,
    /// Ordered list of coordinates
    pub waypoints: Vec<GeoPoint>,
    /// Total distance in kilometers
    pub total_distance_km: f64,
    /// Estimated total duration in minutes
    pub estimated_duration_minutes: u32,
    /// Estimated arrival times for each stop
    pub arrival_times: Vec<u32>, // Minutes from start
    /// Optimization score (lower is better)
    pub optimization_score: f64,
}

/// Route optimizer using nearest-neighbor heuristic and 2-opt improvement
pub struct RouteOptimizer {
    /// Average speed in km/h for duration estimates
    average_speed_kmh: f64,
    /// Time per stop in minutes (loading/unloading)
    service_time_minutes: u32,
}

impl RouteOptimizer {
    /// Create a new route optimizer
    pub fn new() -> Self {
        Self {
            average_speed_kmh: 40.0, // Urban delivery speed
            service_time_minutes: 5,
        }
    }
    
    /// Configure average speed
    pub fn with_speed(mut self, speed_kmh: f64) -> Self {
        self.average_speed_kmh = speed_kmh;
        self
    }
    
    /// Configure service time per stop
    pub fn with_service_time(mut self, minutes: u32) -> Self {
        self.service_time_minutes = minutes;
        self
    }
    
    /// Optimize route for given stops
    /// 
    /// Uses a two-phase approach:
    /// 1. Nearest-neighbor heuristic for initial solution
    /// 2. 2-opt local search for improvement
    pub fn optimize(
        &self,
        start: GeoPoint,
        stops: Vec<(String, GeoPoint)>,
    ) -> Result<OptimizedRoute> {
        if stops.is_empty() {
            return Err(WmsError::RouteOptimization("No stops provided".to_string()));
        }
        
        // Build distance matrix
        let mut all_points: Vec<GeoPoint> = vec![start];
        all_points.extend(stops.iter().map(|(_, p)| *p));
        
        let n = all_points.len();
        let mut distances = vec![vec![0.0; n]; n];
        
        for i in 0..n {
            for j in 0..n {
                distances[i][j] = all_points[i].distance_to(&all_points[j]);
            }
        }
        
        // Phase 1: Nearest-neighbor heuristic
        let mut route = self.nearest_neighbor(&distances);
        
        // Phase 2: 2-opt improvement
        route = self.two_opt_improve(&distances, route);
        
        // Build result
        let stop_order: Vec<String> = route.iter()
            .skip(1) // Skip start depot
            .map(|&i| stops[i - 1].0.clone())
            .collect();
        
        let waypoints: Vec<GeoPoint> = route.iter()
            .map(|&i| all_points[i])
            .collect();
        
        let total_distance = self.calculate_route_distance(&distances, &route);
        let estimated_duration = self.calculate_duration(total_distance, stops.len());
        let arrival_times = self.calculate_arrival_times(&distances, &route);
        
        Ok(OptimizedRoute {
            stop_order,
            waypoints,
            total_distance_km: total_distance,
            estimated_duration_minutes: estimated_duration,
            arrival_times,
            optimization_score: total_distance,
        })
    }
    
    /// Nearest-neighbor heuristic
    fn nearest_neighbor(&self, distances: &[Vec<f64>]) -> Vec<usize> {
        let n = distances.len();
        let mut visited = vec![false; n];
        let mut route = vec![0]; // Start at depot
        visited[0] = true;
        
        for _ in 1..n {
            let current = *route.last().unwrap();
            let mut nearest = None;
            let mut nearest_dist = f64::MAX;
            
            for j in 0..n {
                if !visited[j] && distances[current][j] < nearest_dist {
                    nearest = Some(j);
                    nearest_dist = distances[current][j];
                }
            }
            
            if let Some(next) = nearest {
                route.push(next);
                visited[next] = true;
            }
        }
        
        route
    }
    
    /// 2-opt local search improvement
    fn two_opt_improve(&self, distances: &[Vec<f64>], mut route: Vec<usize>) -> Vec<usize> {
        let n = route.len();
        if n < 4 {
            return route;
        }
        
        let mut improved = true;
        let max_iterations = 1000;
        let mut iterations = 0;
        
        while improved && iterations < max_iterations {
            improved = false;
            iterations += 1;
            
            for i in 1..(n - 2) {
                for j in (i + 1)..(n - 1) {
                    let delta = self.two_opt_delta(distances, &route, i, j);
                    
                    if delta < -0.001 { // Improvement found
                        // Reverse segment between i and j
                        route[i..=j].reverse();
                        improved = true;
                    }
                }
            }
        }
        
        route
    }
    
    /// Calculate improvement delta for 2-opt swap
    fn two_opt_delta(&self, distances: &[Vec<f64>], route: &[usize], i: usize, j: usize) -> f64 {
        let a = route[i - 1];
        let b = route[i];
        let c = route[j];
        let d = route[j + 1];
        
        // Current edges: a-b and c-d
        // After swap: a-c and b-d
        let current = distances[a][b] + distances[c][d];
        let proposed = distances[a][c] + distances[b][d];
        
        proposed - current
    }
    
    /// Calculate total route distance
    fn calculate_route_distance(&self, distances: &[Vec<f64>], route: &[usize]) -> f64 {
        let mut total = 0.0;
        for i in 0..(route.len() - 1) {
            total += distances[route[i]][route[i + 1]];
        }
        total
    }
    
    /// Calculate estimated duration
    fn calculate_duration(&self, distance_km: f64, num_stops: usize) -> u32 {
        let travel_time = (distance_km / self.average_speed_kmh * 60.0) as u32;
        let service_time = (num_stops as u32) * self.service_time_minutes;
        travel_time + service_time
    }
    
    /// Calculate arrival times at each stop
    fn calculate_arrival_times(&self, distances: &[Vec<f64>], route: &[usize]) -> Vec<u32> {
        let mut times = Vec::new();
        let mut current_time: f64 = 0.0;
        
        for i in 0..(route.len() - 1) {
            let dist = distances[route[i]][route[i + 1]];
            let travel_time = dist / self.average_speed_kmh * 60.0;
            current_time += travel_time;
            
            if i > 0 {
                current_time += self.service_time_minutes as f64;
            }
            
            times.push(current_time as u32);
        }
        
        times
    }
}

impl Default for RouteOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_route_optimization() {
        let optimizer = RouteOptimizer::new();
        
        let start = GeoPoint::new(40.7128, -74.0060); // NYC
        let stops = vec![
            ("del1".to_string(), GeoPoint::new(40.7580, -73.9855)), // Times Square
            ("del2".to_string(), GeoPoint::new(40.7484, -73.9857)), // Empire State
            ("del3".to_string(), GeoPoint::new(40.6892, -74.0445)), // Statue of Liberty
        ];
        
        let result = optimizer.optimize(start, stops).unwrap();
        
        assert_eq!(result.stop_order.len(), 3);
        assert!(result.total_distance_km > 0.0);
        assert!(result.estimated_duration_minutes > 0);
    }
    
    #[test]
    fn test_distance_calculation() {
        let p1 = GeoPoint::new(40.7128, -74.0060); // NYC
        let p2 = GeoPoint::new(34.0522, -118.2437); // LA
        
        let distance = p1.distance_to(&p2);
        
        // Should be approximately 3940 km
        assert!(distance > 3900.0 && distance < 4000.0);
    }
}

