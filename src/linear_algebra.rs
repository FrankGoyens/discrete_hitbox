use crate::foundation as dhf;

impl dhf::Point{
    fn normal(&self) -> dhf::Point{
        return dhf::Point{x: -self.y, y: self.x};
    }
    
    fn dot(&self, other: &dhf::Point) -> dhf::DHFloat{
        return self.x*other.x + self.y*other.y;
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
}