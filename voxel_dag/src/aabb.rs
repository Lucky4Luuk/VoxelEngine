use glam::*;

pub struct BoundingBox {
    pub min: Vec3A,
    pub max: Vec3A,
}

impl BoundingBox {
    pub fn new(min: Vec3A, max: Vec3A) -> Self {
        Self {
            min: min,
            max: max,
        }
    }

    //Squares the bounding box, but keeps the current center as the new center
    pub fn square(&mut self) {
        let len = self.max - self.min;
        let mut largest = len.x();
        if len.y() > largest { largest = len.y(); }
        if len.z() > largest { largest = len.z(); }

        let half_len = largest * 0.5;

        let center = self.center();

        self.min.set_x(center.x() - half_len);
        self.max.set_x(center.x() + half_len);

        self.min.set_y(center.y() - half_len);
        self.max.set_y(center.y() + half_len);

        self.min.set_z(center.z() - half_len);
        self.max.set_z(center.z() + half_len);
    }

    pub fn center(&self) -> Vec3A {
        (self.min + self.max) * 0.5
    }
}
