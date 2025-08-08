use crate::{Object, Point, Ray, Vector};


#[derive(Debug, Clone)]
pub struct Intersection<'a> {
    pub object: &'a Object,
    pub t: f32, // distance
    pub point: Point,
    pub normal: Vector,
    pub hit_normal: Vector,
    pub over_point: Point,
    pub reflectv: Vector,
    pub n1: f32, // ca c'est le milieu de depart, c'est de la merde, je veux pas le mettre la
}

impl <'a> Intersection <'a> {
    
    pub fn new(
        object: & 'a Object,
        t: f32,
        point: Point,
        normal: Vector,
        hit_normal: Vector,
        over_point: Point,
        reflectv: Vector,
        n1: f32,
    ) -> Self {
        Intersection {
            object,
            t,
            point,
            normal,
            hit_normal,
            over_point,
            reflectv,
            n1,
        }
    }
}

// trait Intersect have to be implemented in Intersect
pub struct LocalIntersection {
    pub point: Point,
    pub normal: Vector,
    pub t: f32,
}

pub trait Intersect {
    fn intersect(&self, ray: Ray) -> Option<LocalIntersection>;
    fn normal_at(&self, point: Point) -> Vector;
}
