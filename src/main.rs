use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        //.loop_mode(LoopMode::rate_fps(30.0))
        .simple_window(view)
        .run();
}

// for easier conversions 
struct Cartesian {
    x: f32,
    y: f32,
}

fn to_cartesian(r: f32, theta: f32) -> Cartesian {
    // polar to cartesian, with theta = 0 at the down (neutral) position
    let x = r * theta.sin();
    let y = - r * theta.cos();
    Cartesian { x, y }
}


// state variables

struct Model {

    l1: f32,
    theta_1: f32,
    m1: f32,
    v1: f32,
    a1: f32,

    l2: f32,
    theta_2: f32,
    m2: f32,
    v2: f32,
    a2: f32,

    gravity: f32,
    //dampening: f32, // TODO: implement decay
}

// instantiate
fn model(_app: &App) -> Model {

    Model {

        // all necessary values:

        l1: 100.0, // length (m)
        theta_1: 0.5, // angle (radians)
        m1: 1.0, // mass (kg)
        v1: 0.0, // angular velocity (m/s)
        a1: 0.0, // angular acceleration (m/s/s)

        // and for second (child) pendulum
        l2: 100.0,
        theta_2: 1.0,
        m2: 1.0,
        v2: 0.0,
        a2: 0.0,

        // constants

        gravity: 10.0, // (m/s/s)
        //dampening: 0.001,

    }

}

fn ddot_thetas(model: &Model, theta_1: f32, theta_2: f32, dot_theta_1: f32, dot_theta_2: f32) -> (f32, f32) {

    // cache reused values to reduce computation

    let total_mass = model.m1 + model.m2;

    let sin_theta_1 = theta_1.sin();
    //let sin_theta_2 = theta_2.sin();

    let dtheta = theta_1 - theta_2;

    let sin_dtheta = dtheta.sin();
    let cos_dtheta = dtheta.cos();

    let denominator = 2.0 * model.m1 + model.m2 * (1.0 - (2.0 * dtheta).cos());

    // p1 acceleration

    let a1 = 
        - ((model.gravity * 2.0 * total_mass * sin_theta_1) + (model.m2 * model.gravity * (theta_1 - 2.0 * theta_2).sin()) + (2.0 * sin_dtheta * model.m2 * (model.l2 * dot_theta_2 * dot_theta_2 + model.l1 * dot_theta_1 * dot_theta_1 * cos_dtheta))) / (model.l1 * denominator);
        //- model.gravity * theta_1.sin();

        //-(sin_dtheta * (model.m2 * model.l1 * model.l1 * dot_theta_1 * cos_dtheta + model.m2 * model.l2 * dot_theta_1) - model.gravity * (total_mass * sin_theta_1 - model.m2 * sin_theta_2 * cos_dtheta)) / (model.l1 * alpha);

    // p2 acceleration

    let a2 = 
        //- model.gravity * theta_2.sin();
        (2.0 * sin_dtheta * (model.l1 * total_mass * dot_theta_1 * dot_theta_1) + (theta_1.cos() * model.gravity * total_mass) + (model.l2 * model.m2 * dot_theta_2 * dot_theta_2 * cos_dtheta)) / (model.l2 * denominator);
        
        //(sin_dtheta * (total_mass * model.l1 * dot_theta_1 * dot_theta_1 + model.m2 * model.l2 * dot_theta_2 * dot_theta_2 * cos_dtheta) + model.gravity * (total_mass * sin_theta_1 * cos_dtheta - total_mass * sin_theta_2)) / (model.l2 * alpha);

    (a1, a2) // (a1, a2)

}

// runs once a frame (defaults to 60?)

fn update(app: &App, model: &mut Model, _update: Update) {

    // rk4 implementation
     
    let dt = 2.0 * app.duration.since_prev_update.as_secs_f32(); // accounts for variance in update rate

    // current accelerations
    let k1: (f32, f32) = ddot_thetas(model, model.theta_1, model.theta_2, model.v1, model.v2);
    println!("{}, {}", k1.0, k1.1);
    model.v1 += dt * k1.0;
    model.v2 += dt * k1.1;

    model.theta_1 += dt * model.v1;
    model.theta_2 += dt * model.v2;
        
//    let k2: (f32, f32) = ddot_thetas(model,)
//        // new projected velocity 1
//        model.v1 + (dt / 2.0) * k1.0 ,
//        //
//    )
    

}

fn view(app: &App, model: &Model, frame: Frame) {

    let draw = app.draw();

    // cartesian tuples for the endpoint of each pendulum
    // stored relative to each pendulum's origin 
    let p1 = to_cartesian(model.l1, model.theta_1);
    let p2 = to_cartesian(model.l2, model.theta_2); 

    draw.background().color(WHITESMOKE);

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

