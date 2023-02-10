use wasm_bindgen::prelude::*;
use std::f32::consts::PI;
const gravity: f32 = -0.0003;

type Vector2 = [f32; 2];
type Vector4 = [f32; 4];
type Color = Vector4;

fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    [r, g, b, a]
}

pub struct Explosion {
    center: Vector2,
    color: Color,
    ttl: u32,
}

pub struct ParticleSystemState {
    pub count: usize,
    pub positions: Vec<f32>,
    pub velocities: Vec<f32>,
    pub colors: Vec<f32>,
    pub explosions: Vec<Explosion>,
    pub autoexplosions: bool,
}


pub struct ParticleSystem {
    pub state: ParticleSystemState,
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

impl ParticleSystem {
    // TODO remove duplication
    pub fn create_explosion_at(state: &mut ParticleSystemState, x: f32, y: f32) {
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
        state.explosions.clear();

        state.explosions.push(explosion);
        ParticleSystem::reinitialize_particles(state);
    }
    // TODO remove duplication
    fn create_explosion(state: &mut ParticleSystemState) {
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
        state.explosions.push(explosion);
    }
    fn reinitialize_particles(state: &mut ParticleSystemState) {
        let speed_factor = rand::random::<f32>() * 0.023 + 0.002;
        let kind = (rand::random::<f32>() * 6.0) as u8;
        for i in 0..state.count {
            let rnd = (rand::random::<f32>() * state.explosions.len() as f32) as usize;
            let explosion = &state.explosions[rnd];
            let x = state.positions[i * 2];
            let y = state.positions[i * 2 + 1];
            
            
            if state.colors[i * 4 + 3] < 0.1 {

                let particle = Particle::new(&explosion.center, &explosion.color, speed_factor, kind);
                state.positions.splice(i * 2..i * 2 + 2, particle.position);
                state.velocities.splice(i * 2..i * 2 + 2, particle.velocity);
                state.colors.splice(i * 4..i * 4 + 4, particle.color);
            }
        }
    }
    pub fn update(state: &mut ParticleSystemState) {
        let mut is_y = false;

        for (pos, vel) in state.positions.iter_mut().zip(state.velocities.iter_mut()) {
            *pos += *vel;
            if is_y {
                *vel += gravity;
            }
            is_y = !is_y;
        }

        for i in 0..state.count {
            let idx = i * 4;
            let color = state.colors[idx + 3];
            // let new_color;
            if color > 0.9 {
                state.colors[idx + 3] *= 0.9988; 
            } else if state.colors[idx + 3] > 0.8 {
                state.colors[idx + 3] *= 0.993; 
            } else {
                if rand::random::<f32>() < 0.02 {
                    state.colors[idx + 3] = 0.6;
                } else {
                    state.colors[idx + 3] *= 0.92; 
                }
                
            }
            
        }

        state.explosions.retain_mut(|explosion| {
            explosion.ttl -= 1;
            return explosion.ttl > 0;
        });
        if state.explosions.len() == 0  {
            ParticleSystem::create_explosion(state);
        }
        if state.autoexplosions && rand::random::<f32>() < 0.03 {
            ParticleSystem::reinitialize_particles(state);
        }

    }
}
