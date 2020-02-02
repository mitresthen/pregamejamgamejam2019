use std::collections::HashMap;
use std::rc::Rc;

use crate::vector::Vec2;
use crate::transform::Transform;
use crate::game_object::{
    CollisionShape,
    PhysicalObject
};

struct Body {
    velocity: Vec2,
    spin: f32,
    shape: Rc<dyn CollisionShape>,
    transform: Transform,
    radius: f32,
    inv_mass: f32,
    inv_inertia: f32,
    friction: f32,
    src_mask: u32,
    dst_mask: u32,
}

#[derive(Copy, Clone)]
pub struct BodyId {
    id: usize
}

#[derive(Clone)]
struct CollisionPair{
    a: usize,
    b: usize,
    axis: Vec2,
    target_velocity: f32,
    resistance: f32,
    torque_a: f32,
    torque_b: f32,
    r_a: f32,
    r_b: f32,
    depth: f32,
    point: Vec2,
    force_limit: Option<(usize, f32)>,
    unidirectional: bool,
    f_sum: f32,
}

impl CollisionPair {
    fn calculate(ai: usize, bi: usize, a: &Body, b: &Body, point: Vec2, axis: Vec2, depth: f32) -> Self {
        let perp = axis.perpendicular();

        let r_a = perp.dot_product(a.transform.get_translation() - point);
        let r_b = perp.dot_product(b.transform.get_translation() - point);

        let torque_a = r_a * a.inv_mass * a.inv_inertia;
        let torque_b = r_b * b.inv_mass * b.inv_inertia;

        let resistance = a.inv_mass + b.inv_mass + (torque_a * r_a) + (torque_b * r_b);

        let separation = 1.0;

        CollisionPair {
            a: ai,
            b: bi,
            axis: axis,
            depth: depth,
            point,
            torque_a,
            torque_b,
            target_velocity: depth * separation,
            r_a,
            r_b,
            resistance,
            force_limit: None,
            unidirectional: false,
            f_sum: 0.0,
        }
    }
}


pub struct PhysicsSet {
    bodies: Vec<Body>,
    collision_pairs: Vec<CollisionPair>,
    force_sums: Vec<f32>,
    event_axes: HashMap<usize, Vec<Vec2>>
}

impl PhysicsSet {
    pub fn new() -> PhysicsSet {
        PhysicsSet {
            bodies: Vec::new(),
            collision_pairs: Vec::new(),
            force_sums: Vec::new(),
            event_axes: HashMap::new()
        }
    }

    pub fn clear(&mut self) {
        self.bodies.clear();
        self.collision_pairs.clear();
        self.force_sums.clear();
    }

    pub fn add_physics_object(&mut self, physics_object: &dyn PhysicalObject) -> Option<BodyId> {
        if let Some(shape) = physics_object.get_collision_shape() {
            let mut radius : f32 = 0.0;
            for p in shape.get_points() {
                radius = radius.max(p.len());
            }

            let body =
                Body {
                    velocity: *physics_object.get_velocity(),
                    shape,
                    transform: physics_object.get_transform().clone(),
                    radius,
                    inv_mass: physics_object.get_inv_mass(),
                    inv_inertia: physics_object.get_rotatable().map(|r| r.get_inv_inertia()).unwrap_or(0.0),
                    spin: physics_object.get_rotatable().map(|r| r.get_spin()).unwrap_or(0.0),
                    friction: physics_object.get_friction(),
                    dst_mask: physics_object.get_dst_mask(),
                    src_mask: physics_object.get_src_mask(),
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
                if (a.dst_mask & b.dst_mask != 0) {
                    continue
                }
                if ai <= bi {
                    continue
                }
                if a.inv_mass == 0.0 && b.inv_mass == 0.0 {
                    continue
                }

                let distance = (a.transform.get_translation() - b.transform.get_translation()).len_sq();
                let radi_sum = a.radius + b.radius;
                if distance < (radi_sum * radi_sum) {
                    if let Some(result) = a.shape.sat_collide(&a.transform, b.shape.as_ref(), &b.transform) {
                        if (b.src_mask & a.src_mask != 0) {
                            let mut result_vec: Vec<Vec2> = Vec::new();
                            result_vec.push(result.axis);
                            match self.event_axes.get_mut(&ai) {
                                Some(i) => { i.push(result.axis) },
                                None => { self.event_axes.insert(ai, result_vec.clone()); }
                            }
                            match self.event_axes.get_mut(&bi) {
                                Some(i) => { i.push(result.axis) },
                                None => { self.event_axes.insert(bi, result_vec.clone()); }
                            }
                            continue
                        }
                        let manifold_a = a.shape.build_manifold(result.axis * -1.0, &a.transform);
                        let manifold_b = b.shape.build_manifold(result.axis, &b.transform);

                        let manifold = manifold_a.clip(manifold_b, result.axis);

                        for i in 0..manifold.point_count {
                            let normal_cp = CollisionPair::calculate(ai, bi, a, b, manifold.points[i], result.axis, result.depth);
                            let normal_id = self.collision_pairs.len();
                            self.collision_pairs.push(normal_cp);
                            self.force_sums.push(0.0);

                            let mut friction_cp = CollisionPair::calculate(ai, bi, a, b, manifold.points[i], result.axis.perpendicular(), 0.0);

                            let friction_factor = (a.friction * b.friction).sqrt();
                            friction_cp.unidirectional = true;
                            friction_cp.force_limit = Some((normal_id, friction_factor));
                            self.collision_pairs.push(friction_cp);
                            self.force_sums.push(0.0);
                        }
                    }
                }
            }
        }

    }

    pub fn iterate(&mut self) {
        for i in 0..self.collision_pairs.len() {
            let cp = &self.collision_pairs[i];

            let a = &self.bodies[cp.a];
            let b = &self.bodies[cp.b];

            let ma = a.inv_mass;
            let mb = b.inv_mass;

            let v_a = a.velocity.dot_product(cp.axis) - (a.spin * cp.r_a);
            let v_b = b.velocity.dot_product(cp.axis) - (b.spin * cp.r_b);

            let delta_v = cp.target_velocity + v_b - v_a;

            if delta_v > 0.0 || cp.unidirectional {
                let mut f = delta_v / cp.resistance;

                if let Some((ref_id, factor)) = cp.force_limit {
                    let limit = (self.force_sums[ref_id] * factor).abs();
                    let total = self.force_sums[i] + f;

                    let adjust =
                        if total > limit {
                            total - limit
                        } else if total < -limit {
                            total + limit
                        } else {
                            0.0
                        };

                    f -= adjust; 
                }

                self.force_sums[i] += f;


                self.bodies[cp.a].velocity += cp.axis * f * ma;
                self.bodies[cp.b].velocity -= cp.axis * f * mb;

                self.bodies[cp.a].spin -= cp.torque_a * f;
                self.bodies[cp.b].spin += cp.torque_b * f;
            }
        }
    }

    pub fn get_velocity(&self, id: BodyId) -> Vec2 {
        self.bodies.get(id.id).unwrap().velocity        
    }

    pub fn get_spin(&self, id: BodyId) -> f32 {
        self.bodies.get(id.id).unwrap().spin        
    }

    pub fn get_collision_points(&self) -> Vec<Vec2> {
        self.collision_pairs.iter()
            .map(|cp| cp.point)
            .collect()
    }

    pub fn get_collision_axes_for_body(&self, id: BodyId) -> Vec<Vec2> {
        let mut axes_for_id: Vec<Vec2> = match self.event_axes.get(&id.id) {
            Some(i) => { i.clone() },
            None => { Vec::new()},
        };
        let mut pairs: Vec<Vec2> = self.collision_pairs.iter()
            .filter(|cp| cp.a == id.id || cp.b == id.id)
            .filter(|cp| !cp.unidirectional)
            .map(|cp| cp.axis.clone() * if cp.b == id.id { -1.0 } else { 1.0 })
            .collect();

        axes_for_id.append(&mut pairs);
        axes_for_id
    }
}
