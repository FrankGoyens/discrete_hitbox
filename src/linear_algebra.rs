use crate::foundation as dhf;

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
        
        return if boundary_direction_dot < 0. {BoundarySituation::Inside} else { BoundarySituation::Outside};
    }
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
        assert_eq!(BoundarySituation::Outside, given_boundary.situate(&dhf::Point{x: -1., y: 0.5}));
        assert_eq!(BoundarySituation::On, given_boundary.situate(&dhf::Point{x: 0., y: 0.5}));
        assert_eq!(BoundarySituation::Inside, given_boundary.situate(&dhf::Point{x: 1., y: 0.5}));
    }
}