use crate::vector::Vec2;

use crate::game_object::{
    CollisionShape,
    PhysicalObject
};

struct Body {
    velocity: Vec2,
    rotation: f32,
    shape: Box<dyn CollisionShape>,
    center: Vec2,
    radius: f32,
    inv_mass: f32,
    inv_intertia: f32,
}

#[derive(Copy, Clone)]
pub struct BodyId {
    id: usize
}

struct CollisionPair{
    a: usize,
    b: usize,
    axis: Vec2,
    resistance: f32,
    depth: f32,
    point: Vec2,
}


pub struct PhysicsSet {
    bodies: Vec<Body>,
    collision_pairs: Vec<CollisionPair>,
}

impl PhysicsSet {
    pub fn new() -> PhysicsSet {
        PhysicsSet {
            bodies: Vec::new(),
            collision_pairs: Vec::new()
        }
    }

    pub fn clear(&mut self) {
        self.bodies.clear();
        self.collision_pairs.clear();
    }

    pub fn add_physics_object(&mut self, physics_object: &dyn PhysicalObject) -> Option<BodyId> {
        if let Some(shape) = physics_object.get_bounding_box() {
            let mut center = Vec2::new();
            for p in shape.get_points() {
                center = center + *p;
            }
            center = center * (1.0 / (shape.get_points().len() as f32));

            let mut radius : f32 = 0.0;
            for p in shape.get_points() {
                radius = radius.max((*p - center).len());
            }


            let body =
                Body {
                    velocity: *physics_object.get_velocity(),
                    shape,
                    center,
                    radius,
                    inv_mass: physics_object.get_inv_mass(),
                    inv_intertia: physics_object.get_rotatable().map(|r| r.get_inv_intertia()).unwrap_or(0.0),
                    rotation: physics_object.get_rotatable().map(|r| r.get_rotation()).unwrap_or(0.0),
                };

            self.bodies.push(body);

            Some(BodyId { id: self.bodies.len() - 1})
        } else {
            None
        }
    }

    pub fn find_collision_pairs(&mut self) {
        for (ai, a) in self.bodies.iter().enumerate() {
            for (bi, b) in self.bodies.iter().enumerate() {
                if ai == bi {
                    continue
                }
                if a.inv_mass == 0.0 && b.inv_mass == 0.0 {
                    continue
                }
                let distance = (a.center - b.center).len_sq();
                let radi_sum = a.radius + b.radius;
                if distance < (radi_sum * radi_sum) {
                    if let Some(result) = a.shape.sat_collide(b.shape.as_ref()) {

                        let manifold_a = a.shape.build_manifold(result.axis);
                        let manifold_b = b.shape.build_manifold(result.axis * -1.0);

                        let manifold = manifold_a.clip(manifold_b, result.axis);

                        let perp = result.axis.perpendicular();

                        for i in 0..manifold.point_count {
                            let r_a = perp.dot_product(a.center - manifold.points[i]);
                            let r_b = perp.dot_product(b.center - manifold.points[i]);

                            let resistance =
                                a.inv_mass +
                                b.inv_mass +
                                (r_a * r_a * a.inv_mass * a.inv_intertia) +
                                (r_b * r_b * b.inv_mass * b.inv_intertia);

                            let collision_pair =
                                CollisionPair {
                                    a: ai,
                                    b: bi,
                                    axis: result.axis,
                                    depth: result.depth,
                                    point: manifold.points[i],
                                    resistance,
                                };

                            self.collision_pairs.push(collision_pair);
                        }
                    }
                }
            }
        }
    }

    pub fn iterate(&mut self) {
        for cp in self.collision_pairs.iter() {
            let a = &self.bodies[cp.a];
            let b = &self.bodies[cp.b];



            let ma = a.inv_mass;
            let mb = b.inv_mass;

            let v_a = a.velocity.dot_product(cp.axis);
            let v_b = b.velocity.dot_product(cp.axis);

            let delta_v = v_a - v_b;

            if delta_v < 0.0 {
                let f = delta_v / cp.resistance;

                self.bodies[cp.a].velocity -= cp.axis * f * ma;
                self.bodies[cp.b].velocity += cp.axis * f * mb;
            }
        }
    }

    pub fn get_velocity(&self, id: BodyId) -> Vec2 {
        self.bodies.get(id.id).unwrap().velocity        
    }

}
