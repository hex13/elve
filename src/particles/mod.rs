use wasm_bindgen::prelude::*;
use std::f32::consts::PI;
const gravity: f32 = -0.0003;

type Vector2 = [f32; 2];
type Vector4 = [f32; 4];
type Color = Vector4;

fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    [r, g, b, a]
}
struct Explosion {
    center: Vector2,
    color: Color,
    ttl: u32,
}

impl Explosion {
    
}

#[wasm_bindgen]
pub struct ParticleSystem {
    count: usize,
    positions: Vec<f32>,
    velocities: Vec<f32>,
    colors: Vec<f32>,
    explosions: Vec<Explosion>,
    autoexplosions: bool,
}

struct Particle {
    position: Vector2,
    velocity: Vector2,
    color: Color,
}

impl Particle {
    fn new(center: &Vector2, color: &Color, speed_factor: f32, kind: u8) -> Particle {
        let speed = rand::random::<f32>() * speed_factor + 0.0002;
        let angle = rand::random::<f32>() * 2. * PI;
        let mut _color = color.clone();
        _color[3] -= rand::random::<f32>() * 0.1;
        let velocity;
        let position;
        if kind == 1 {
            velocity = [angle.cos() * speed * 0.1, angle.sin() * speed * 0.2 + 0.024];
            position = center.clone();
        } else if kind == 2 {
            let angle2 = ((angle * 3.0) as i32) as f32;
            velocity = [angle2.cos() * speed, angle2.sin() * speed + 0.01];
            position = [
                center[0] + rand::random::<f32>() * 0.016 - 0.008, 
                center[1] + rand::random::<f32>() * 0.016 - 0.008, 
            ];
        } else if kind == 3 {
            // velocity = [angle.cos() * rand::random::<f32>() * 0.01, 0.02 + rand::random::<f32>() * 0.01];
            velocity = [0.01 * angle.cos() * angle, 0.01 * angle.sin() * angle];
            position = [
                center[0] + rand::random::<f32>() * 0.016 - 0.008, 
                center[1] + rand::random::<f32>() * 0.016 - 0.008, 
            ];

        } else {
            velocity = [angle.cos() * speed, angle.sin() * speed + 0.01];
            position = [
                center[0] + rand::random::<f32>() * 0.010 - 0.005, 
                center[1] + rand::random::<f32>() * 0.010 - 0.005, 
            ];
        }
        Particle { 
            position,
            velocity,
            color: _color,
        }
    }
}

#[wasm_bindgen]
impl ParticleSystem {
    #[wasm_bindgen(constructor)]
    pub fn new(count: usize) -> ParticleSystem {
        let unit = 0.2;
        let positions = vec![0.0; count * 2];
        let velocities = vec![0.0; count * 2];
        let colors = vec![0.0; count * 4];
        let explosions = Vec::new();
        ParticleSystem { count, positions, velocities, colors, explosions, autoexplosions: false }
    }
    pub fn positions(&self) -> *const f32 {
        &self.positions[0]
    }
    pub fn colors(&self) -> *const f32 {
        &self.colors[0]
    }
    pub fn set_autoexplosions(&mut self, value: bool) {
        self.autoexplosions = value;
    }
    // TODO remove duplication
    pub fn create_explosion_at(&mut self, x: f32, y: f32) {
        
        let mut color = rgba(
            rand::random::<f32>() * 0.5 + 0.5,
            rand::random::<f32>() * 0.5 + 0.5,
            rand::random::<f32>() * 0.5 + 0.5,
            1.0
        );
        let component = (rand::random::<f32>() * 3.0) as usize;
        color[component] = 1.0;
        let explosion = Explosion {
            center: [x, y],
            color,
            ttl: 100,
        };
        self.explosions.clear();

        self.explosions.push(explosion);
        self.reinitialize_particles();
    }
    // TODO remove duplication
    pub fn create_explosion(&mut self) {
        let mut color = rgba(
            rand::random::<f32>() * 0.5 + 0.5,
            rand::random::<f32>() * 0.5 + 0.5,
            rand::random::<f32>() * 0.5 + 0.5,
            1.0
        );
        let component = (rand::random::<f32>() * 3.0) as usize;
        color[component] = 1.0;
        let explosion = Explosion {
            center: [rand::random::<f32>() * 1.0 - 0.5, rand::random::<f32>() * 1.0 - 0.75],
            color,
            ttl: 1,
        };
        self.explosions.push(explosion);
    }
    pub fn reinitialize_particles(&mut self) {
        let speed_factor = rand::random::<f32>() * 0.023 + 0.002;
        let kind = (rand::random::<f32>() * 6.0) as u8;
        for i in 0..self.count {
            let rnd = (rand::random::<f32>() * self.explosions.len() as f32) as usize;
            let explosion = &self.explosions[rnd];
            let x = self.positions[i * 2];
            let y = self.positions[i * 2 + 1];
            
            
            if self.colors[i * 4 + 3] < 0.1 {

                let particle = Particle::new(&explosion.center, &explosion.color, speed_factor, kind);
                self.positions.splice(i * 2..i * 2 + 2, particle.position);
                self.velocities.splice(i * 2..i * 2 + 2, particle.velocity);
                self.colors.splice(i * 4..i * 4 + 4, particle.color);
            }
        }
    }
    pub fn debug(&mut self) -> usize {
        self.explosions.len()
    }
    pub fn update(&mut self) {
        let mut is_y = false;

        for (pos, vel) in self.positions.iter_mut().zip(self.velocities.iter_mut()) {
            *pos += *vel;
            if is_y {
                *vel += gravity;
            }
            is_y = !is_y;
        }

        for i in 0..self.count {
            let idx = i * 4;
            let color = self.colors[idx + 3];
            // let new_color;
            if color > 0.9 {
                self.colors[idx + 3] *= 0.9988; 
                // self.velocities[i * 2 + 1] += rand::random::<f32>() * 0.0001; // nice pseudo-3d effect
                // self.velocities[i * 2 + 0] += rand::random::<f32>() * 0.0004 - 0.0002; // nice pseudo-3d effect

            } else if self.colors[idx + 3] > 0.8 {
                self.colors[idx + 3] *= 0.993; 
            } else {
                if rand::random::<f32>() < 0.02 {
                    self.colors[idx + 3] = 0.6;
                } else {
                    self.colors[idx + 3] *= 0.92; 
                }
                
            }
            
        }

        self.explosions.retain_mut(|explosion| {
            explosion.ttl -= 1;
            return explosion.ttl > 0;
        });
        if self.explosions.len() == 0  {
            self.create_explosion();
        }
        if self.autoexplosions && rand::random::<f32>() < 0.03 {
            self.reinitialize_particles();
        }

    }
}
