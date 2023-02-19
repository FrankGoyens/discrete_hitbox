pub type DHFloat = f32;

pub fn float_max(first: DHFloat, second: DHFloat) -> DHFloat{
    return float_ord::FloatOrd(first).max(float_ord::FloatOrd(second)).0;
}

pub fn is_close_ex(first: DHFloat, second: DHFloat,  rel_tol: Option<DHFloat>, abs_tol: Option<DHFloat>) -> bool{
    let chosen_rel_tol = rel_tol.unwrap_or(1e-9);
    let chosen_abs_tol = abs_tol.unwrap_or(1e-6);
    let applied_rel_tol = chosen_rel_tol * float_max(first.abs(), second.abs());
    return (first-second).abs() <= float_max(applied_rel_tol, chosen_abs_tol);
}

pub fn is_close(first: DHFloat, second: DHFloat) -> bool{
    return is_close_ex(first, second, None, None);
}

pub fn float_le(first: DHFloat, second: DHFloat) -> bool{
    return is_close(first, second) || first < second;
}

pub fn float_gt(first: DHFloat, second: DHFloat) -> bool{
    return !is_close(first, second) && first > second;
}

#[derive(Debug, Clone)]
pub struct Point{
    pub x:DHFloat,
    pub y:DHFloat,
}

impl PartialEq for Point{
    fn eq(&self, other: &Point) -> bool{
        return is_close_ex(self.x, other.x, None, None)
            && is_close_ex(self.y, other.y, None, None);
    }

    fn ne(&self, other: &Point) -> bool{
        return !self.eq(other);
    }
}

#[derive(Debug, Clone)]
pub struct Line{
    pub start: Point,
    pub end: Point,
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_float_max(){
        assert_eq!(2., float_max(1., 2.));
        assert_eq!(3., float_max(2., 3.));
        assert_eq!(2., float_max(2., 1.));
        assert_eq!(3., float_max(3., 2.));
    }

    #[test]
    fn test_is_close(){
        assert!(!is_close_ex(1., 2., None, None));
        assert!(is_close_ex(2., 2., None, None));
        assert!(!is_close_ex(3., 2., None, None));
    }

    #[test]
    fn test_is_close_absolute(){
        assert!(!is_close_ex(0.1, 0., None, Some(0.001)));
        assert!(!is_close_ex(0.01, 0., None, Some(0.001)));
        assert!(is_close_ex(0.001, 0., None, Some(0.001)));
        assert!(is_close_ex(0.0001, 0., None, Some(0.001)));
        assert!(is_close_ex(0.00001, 0., None, Some(0.001)));
    }

    #[test]
    fn test_is_close_relative(){
        assert!(is_close_ex(100., 100., Some(1e-10), None));
        assert!(!is_close_ex(100., 100.1, Some(1e-10), None));
        assert!(is_close_ex(1_000_000_000., 1_000_000_000.1, Some(1e-10), None));
    }
}