mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Rectangle {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

#[wasm_bindgen]
impl Rectangle {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Rectangle { x,y,w,h }
    }

    pub fn contains(&self, p: Point) -> bool {
        p.x > self.x 
            && p.x < self.w+self.x 
            && p.y > self.y 
            && p.y < self.y+self.h
    }

    pub fn intersects(&self, r: Rectangle) -> bool {
        self.x > r.x + r.w ||
         self.x + self.w < r.x ||
         self.y > r.y + r.h ||
         self.y + self.h < r.y
    }
}


#[wasm_bindgen]
pub struct QuadTree {
    boundary: Rectangle,
    capacity: usize,
    points: Vec<Point>,
    divided: bool,
    ne: Option<Box<QuadTree>>,
    nw: Option<Box<QuadTree>>,
    sw: Option<Box<QuadTree>>,
    se: Option<Box<QuadTree>>,
}

#[wasm_bindgen]
impl QuadTree {
    pub fn new(boundary: Rectangle, capacity: Option<usize>) -> Self {
        let points: Vec<Point> = Vec::new();
        QuadTree {
            boundary,
            capacity: match capacity { Some(c) => c, None => 4}, 
            points,
            divided: false,
            ne: None,
            nw: None,
            sw: None,
            se: None
        }
    }

    pub fn get_points(&self) -> *const Point {
        self.points.as_ptr()
    }

    pub fn is_divided(&self) -> bool {
        self.divided
    }

    pub fn subdivide(&mut self) {
        let neb = Rectangle::new(
            self.boundary.x + self.boundary.w/2.0,
            self.boundary.y,
            self.boundary.w/2.0, 
            self.boundary.h/2.0
        );
        let new_ne = Box::new(QuadTree::new(neb, Some(self.capacity)));
        self.ne = Option::Some(new_ne);

        let nwb = Rectangle::new(
            self.boundary.x,
            self.boundary.y,
            self.boundary.w/2.0, 
            self.boundary.h/2.0
        );
        let new_nw = Box::new(QuadTree::new(nwb, Some(self.capacity)));
        self.ne = Option::Some(new_nw);

        let swb = Rectangle::new(
            self.boundary.x,
            self.boundary.y + self.boundary.h/2.0,
            self.boundary.w/2.0, 
            self.boundary.h/2.0
        );
        let new_sw = Box::new(QuadTree::new(swb, Some(self.capacity)));
        self.sw = Option::Some(new_sw);

        let seb = Rectangle::new(
            self.boundary.x + self.boundary.w/2.0,
            self.boundary.y + self.boundary.h/2.0,
            self.boundary.w/2.0, 
            self.boundary.h/2.0
        );
        let new_se = Box::new(QuadTree::new(seb, Some(self.capacity)));
        self.se = Option::Some(new_se);

        self.divided = true;
    }

    pub fn insert(&mut self, p: Point) {
        if !self.boundary.contains(p){
            return;
        }
        if self.points.len() < self.capacity {
            self.points.push(p);
        } else {
            self.subdivide();

            let ins = |q: &mut Option<Box<QuadTree>> , p: Point| {
                match q {
                    Some(qt) => qt.insert(p), 
                    None => ()
                }
            };
            ins(&mut self.ne, p);
            ins(&mut self.nw, p);
            ins(&mut self.sw, p);
            ins(&mut self.se, p);
        }
    }
}