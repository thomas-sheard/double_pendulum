use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run()
}

// store defining degrees of freedom as a state object
#[derive(Copy, Clone)]
struct State {
    theta_1: f32,
    theta_2: f32,
    dot_theta_1: f32,
    dot_theta_2: f32,
}

// implement scalar vector multiplication, division, and vector addition for rk4 

impl std::ops::Mul<f32> for State {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            theta_1: self.theta_1 * rhs,
            theta_2: self.theta_2 * rhs,
            dot_theta_1: self.dot_theta_1 * rhs,
            dot_theta_2: self.dot_theta_2 * rhs,
        }
    }
}

impl std::ops::Div<f32> for State {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self {
            theta_1: self.theta_1 / rhs,
            theta_2: self.theta_2 / rhs,
            dot_theta_1: self.dot_theta_1 / rhs,
            dot_theta_2: self.dot_theta_2 / rhs,
        }
    }
}

impl std::ops::Add<State> for State {
    type Output = Self;

    fn add(self, other: State) -> Self {
        Self {
            theta_1: self.theta_1 + other.theta_1,
            theta_2: self.theta_2 + other.theta_2,
            dot_theta_1: self.dot_theta_1 + other.dot_theta_1,
            dot_theta_2: self.dot_theta_2 + other.dot_theta_2,
        }
    }
}

// for easier conversion between polar and cartesian
struct Cartesian {
    x: f32,
    y: f32,
}

fn to_cartesian(r: f32, theta: f32) -> Cartesian {
    // polar to cartesian
    let x = r * theta.sin();
    let y = - r * theta.cos();

    Cartesian { x, y }
}

fn derivatives(state: &State, model: &Model) -> State {

    // cache reused values for memory
    
    let g = model.gravity;
    let m1 = model.m1;
    let m2 = model.m2;
    let l1 = model.l1;
    let l2 = model.l2;

    let dot_theta_1 = state.dot_theta_1;
    let dot_theta_2 = state.dot_theta_2;

    // cache calculated values for efficiency

    let mratio = m2 / m1;
    let mrat_plus = mratio + 1.0;
    let lratio = l2 / l1;
    let gamma = g / l1;
    let dtheta = state.theta_1 - state.theta_2;

    let sin_theta_1 = state.theta_1.sin();
    let sin_theta_2 = state.theta_2.sin();

    let sin_dtheta = dtheta.sin();
    let cos_dtheta = dtheta.cos();

    let denominator = 1.0 + (mratio * sin_dtheta * sin_dtheta);

    // equations from uni edinburgh (page 30-31):
    // https://www2.ph.ed.ac.uk/~dmarendu/MVP/DoublePendulumTutorial.pdf

    let num_1 = (mrat_plus * gamma * sin_theta_1) + (mratio * lratio * dot_theta_2 * dot_theta_2 * sin_dtheta) + (mratio * cos_dtheta * (dot_theta_1 * dot_theta_1 * sin_dtheta - gamma * sin_theta_2));

    let ddot_theta_1 = - num_1 / denominator;

    let num_2 = mrat_plus * (dot_theta_1 * dot_theta_1 * sin_dtheta - gamma * sin_theta_2) + cos_dtheta * (mrat_plus * gamma * sin_theta_1 + mratio * lratio * dot_theta_2 * dot_theta_2 * sin_dtheta);

    let ddot_theta_2 = num_2 / (lratio * denominator);

    State { 
        theta_1: dot_theta_1,
        theta_2: dot_theta_2,
        dot_theta_1: ddot_theta_1,
        dot_theta_2: ddot_theta_2,
    }

}

fn rk4(state: &State, model: &Model, dt: f32) -> State {

    let k1 = derivatives(state, model) * dt;

    let k2_state = *state + k1 * 0.5;
    let k2 = derivatives(&k2_state, model) * dt;

    let k3_state = *state + k2 * 0.5;
    let k3 = derivatives(&k3_state, model) * dt;
    
    let k4_state = *state + k3;
    let k4 = derivatives(&k4_state, model) * dt;

    *state + (k1 + k2 * 2.0 + k3 * 2.0 + k4) / 6.0
}


struct Model {
    state: State,

    l1: f32,
    l2: f32,

    m1: f32,
    m2: f32,

    gravity: f32,
    // dampening: f32,

    path: Vec<Point2>,
    max_path_length: usize,
}

fn model(_app: &App) -> Model {
    Model {

        state: State {
            // initial displacements
            theta_1: 0.0,
            theta_2: 2.0,

            // initial velocities ('kick')
            dot_theta_1: 0.0,
            dot_theta_2: 0.0,
        },

        l1: 1.0,
        l2: 1.0,

        m1: 1.0,
        m2: 1.0,

        gravity: 10.0,

        path: Vec::new(),
        max_path_length: 500,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {

    // scalar on dt for visualisation speed
    let dt = 1.0 * app.duration.since_prev_update.as_secs_f32();

    // perform rk4 state update
    model.state = rk4(&model.state, model, dt);

    // print current angles
    //println!("[{}, {}]", model.state.theta_1, model.state.theta_2);

    let p1 = to_cartesian(100.0 * model.l1, model.state.theta_1);
    let p2 = to_cartesian(100.0 * model.l2, model.state.theta_2);
    model.path.push(pt2(p1.x + p2.x, p1.y + p2.y));

    if model.path.len() > model.max_path_length {
        model.path.remove(0);
    }

}

fn view(app: &App, model: &Model, frame: Frame) {

    let draw = app.draw();

    let p1 = to_cartesian(100.0 * model.l1, model.state.theta_1);
    let p2 = to_cartesian(100.0 * model.l2, model.state.theta_2);

    draw.background().color(WHITESMOKE);

    // draw trace first for layering

    draw.polyline()
        .color(CADETBLUE)
        .stroke_weight(2.0)
        .points(model.path.iter().cloned());

    // origin
    
    draw.ellipse()
        .color(GRAY)
        .radius(7.0)
        .x_y(0.0, 0.0);

    // to pendulum 1

    draw.line()
        .start(pt2(0.0, 0.0))
        .end(pt2(p1.x, p1.y)) // line.end() takes a nannou pt2 object
        .weight(4.0)
        .color(GRAY);

    draw.ellipse()
        .color(GRAY)
        .radius(7.0)
        .x_y(p1.x, p1.y); // ellipse origin just takes (x, y) from Cartesian

    // to pendulum 2

    draw.line()
        .start(pt2(p1.x, p1.y))
        .end(pt2(p1.x + p2.x, p1.y + p2.y)) // offset from endpoint of p1
        .weight(4.0)
        .color(GRAY);

    draw.ellipse()
        .color(GRAY)
        .radius(7.0)
        .x_y(p1.x + p2.x, p1.y + p2.y);

    draw.to_frame(app, &frame).unwrap();

}
