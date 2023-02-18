use std::vec::Vec;
use crate::foundation as dhf;

pub fn make_rectangle(width: dhf::DHFloat, height: dhf::DHFloat) -> [dhf::Point; 4]{
    let half_width = width/2.;
    let half_height = height/2.;
    return [
        dhf::Point{x: -half_width,y: -half_height},
        dhf::Point{x:half_width, y:-half_height},
        dhf::Point{x:half_width, y:half_height},
        dhf::Point{x:-half_width, y:half_height}];
}

pub fn expand_lines(points: &Vec<dhf::Point>) -> Vec<dhf::Line>{
    let mut lines = Vec::new();
    if points.len() <= 1 { 
        return lines;
    }
    let mut points_iter = points.iter();
    let mut previous = points_iter.next().unwrap();
    for point in points_iter{
        lines.push(dhf::Line{start: previous.clone(), end: point.clone()});
        previous = point;
    }
    lines.push(dhf::Line{start: previous.clone(), end: points.first().unwrap().clone()});
    return lines;
}