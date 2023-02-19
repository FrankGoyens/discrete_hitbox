use std::vec::Vec;
use std::ops::Sub;
use crate::{foundation as dhf, shape_factory};

#[derive(Debug, PartialEq, Eq)]
pub enum BoundarySituation{
    Inside,
    Outside,
    On,
}

impl dhf::Point{
    fn normal(&self) -> dhf::Point{
        return dhf::Point{x: -self.y, y: self.x};
    }
    
    fn dot(&self, other: &dhf::Point) -> dhf::DHFloat{
        return self.x*other.x + self.y*other.y;
    }

    fn situate(&self, other: &dhf::Point) -> BoundarySituation{
        let normal = self.normal();
        let boundary_direction_dot = normal.dot(other);
        if dhf::is_close(boundary_direction_dot, 0.){
            return BoundarySituation::On;
        }
        
        return if boundary_direction_dot < 0. {BoundarySituation::Outside} else { BoundarySituation::Inside};
    }
}

impl Sub<&dhf::Point> for &dhf::Point{
    type Output = dhf::Point;

    fn sub(self, rhs: &dhf::Point) -> Self::Output {
        return dhf::Point{x: self.x - rhs.x, y: self.y - rhs.y};
    }
}

enum Adjustment{
    Increment,
    Decrement,
    Skip,
    OnBoundary
}

fn determine_winding_number_adjustment(start: &dhf::Point, end: &dhf::Point, point: &dhf::Point) -> Adjustment{
    if dhf::float_le(start.y, point.y){
        if dhf::float_gt(end.y, point.y){
            let boundary_situation = (end - start).situate(&(point - start)); 
            match boundary_situation{
                BoundarySituation::On => return Adjustment::OnBoundary,
                BoundarySituation::Inside => return Adjustment::Increment,
                BoundarySituation::Outside => return Adjustment::Skip
            }
        }
    }else{
        if dhf::float_le(end.y, point.y){
            let boundary_situation = (end - start).situate(&(point - start)); 
            match boundary_situation{
                BoundarySituation::On => return Adjustment::OnBoundary,
                BoundarySituation::Inside => return Adjustment::Skip,
                BoundarySituation::Outside => return Adjustment::Decrement
            }
        }
    }
    return Adjustment::Skip;
}

pub fn calc_winding_number(point: &dhf::Point, polygon: &Vec<dhf::Point>) -> i32{
    let mut winding_number = 0;
    if polygon.is_empty(){
        return winding_number;
    }
    let mut polygon_iter = polygon.iter();
    let mut previous_point = polygon_iter.next().unwrap();
    for current_point in polygon_iter{
        match determine_winding_number_adjustment(previous_point,current_point, point){
            Adjustment::Increment => winding_number+=1,
            Adjustment::Decrement => winding_number-=1,
            Adjustment::Skip => (),
            Adjustment::OnBoundary => return 0
        }
        previous_point = current_point;
    }

    let first_point = polygon.first().unwrap();
    match determine_winding_number_adjustment(previous_point,first_point, point){
        Adjustment::Increment => winding_number+=1,
        Adjustment::Decrement => winding_number-=1,
        Adjustment::Skip => (),
        Adjustment::OnBoundary => return 0
    }
    return winding_number;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_normal(){
        let given_point = dhf::Point{x: 2., y:3.};
        assert_eq!(dhf::Point{x:-3., y:2.}, given_point.normal());
    }

    #[test]
    fn test_dot(){
        let given_point = dhf::Point{x: 2., y: 3.};
        assert!(dhf::is_close(2., given_point.dot(&dhf::Point{x: 1., y: 0.})));
        assert!(dhf::is_close(3., given_point.dot(&dhf::Point{x: 0., y: 1.})));

        assert!(dhf::is_close(0., given_point.dot(&given_point.normal())));
    }

    #[test]
    fn test_boundary_situation(){
        let given_boundary = dhf::Point{x: 0.,y: 1.};
        assert_eq!(BoundarySituation::Inside, given_boundary.situate(&dhf::Point{x: -1., y: 0.5}));
        assert_eq!(BoundarySituation::On, given_boundary.situate(&dhf::Point{x: 0., y: 0.5}));
        assert_eq!(BoundarySituation::Outside, given_boundary.situate(&dhf::Point{x: 1., y: 0.5}));
    }

    #[test]
    fn test_calc_empty_shape_winding_number(){
        assert_eq!(0, calc_winding_number(&dhf::Point{x: 0., y:0.}, &vec![]));
        assert_eq!(0, calc_winding_number(&dhf::Point{x: 0., y:0.}, &vec![dhf::Point{x: 1., y: 1.}]));
    }

    #[test]
    fn test_calc_rectangle_winding_number(){
        let given_shape = shape_factory::make_rectangle(12., 10.);
        // inside the shape
        assert_eq!(1, calc_winding_number(&dhf::Point{x: 0., y: 0.}, &given_shape.clone().into_iter().collect()));
        
        // outside the shape
        assert_eq!(0, calc_winding_number(&dhf::Point{x: 11., y: 5.}, &given_shape.clone().into_iter().collect()));
        assert_eq!(0, calc_winding_number(&dhf::Point{x: -11., y: 5.}, &given_shape.clone().into_iter().collect()));
        
        // on the shape boundary
        assert_eq!(0, calc_winding_number(&dhf::Point{x: 6., y: 5.}, &given_shape.clone().into_iter().collect()));
        assert_eq!(0, calc_winding_number(&dhf::Point{x: -6., y: 5.}, &given_shape.clone().into_iter().collect()));
    }

    #[test]
    fn test_calc_concave_poly_winding_number(){
        let given_shape = vec![dhf::Point{x: 0., y: 0.}, 
            dhf::Point{x: 10., y: 0.},
            dhf::Point{x: 10., y: 12.},
            dhf::Point{x: 5., y: 12.},
            dhf::Point{x: 5., y: 3.},
            dhf::Point{x: 3., y: 3.},
            dhf::Point{x: 3., y: 12.},
            dhf::Point{x: 0., y: 12.}];
        // inside the shape
        assert_eq!(1, calc_winding_number(&dhf::Point{x: 4., y: 2.}, &given_shape));
        
        // outside the shape
        assert_eq!(0, calc_winding_number(&dhf::Point{x: 11., y: 5.}, &given_shape));
        assert_eq!(0, calc_winding_number(&dhf::Point{x: -1., y: 5.}, &given_shape));
        
        // on the shape boundary
        assert_eq!(0, calc_winding_number(&dhf::Point{x: 10., y: 12.}, &given_shape));
        assert_eq!(0, calc_winding_number(&dhf::Point{x: 0., y: 0.}, &given_shape));
    }
}