use geo::Point;
pub trait TourStrategy {
    fn build_tour(&self, points: &Vec<(f32, f32)>, hull_points: &Vec<Point<f32>>) -> Vec<usize>;
}

pub struct Tour<T: TourStrategy> {
    tour_strategy: T,
}

pub struct GreedyStrategy;
pub struct CheapestInsertionStrategy;

impl<T: TourStrategy> Tour<T> {
    pub fn new(tour_strategy: T) -> Self {
        Self { tour_strategy }
    }
    pub fn tour(&self, points: &Vec<(f32, f32)>, hull_points: &Vec<Point<f32>>) -> Vec<usize> {
        self.tour_strategy.build_tour(points, hull_points)
    }
}
