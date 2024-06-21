static SIGMA: f64 = 10.0;
static RHO: f64 = 28.0;
static BETA: f64 = 8.0 / 3.0;
// static dt: f64 = 0.001;

// void update_state(double *x, double *y, double *z)
// {
//     double dx = SIGMA * (*y - *x) * dt;
//     double dy = (*x * (RHO - *z) - *y) * dt;
//     double dz = (*x * *y - BETA * *z) * dt;

//     *x += dx;
//     *y += dy;
//     *z += dz;
// }
pub fn lorenz(x: &mut f64, y: &mut f64, z: &mut f64, dt: f64) {
    let dx = SIGMA * (*y - *x) * dt;
    let dy = (*x * (RHO - *z) - *y) * dt;
    let dz = (*x * *y - BETA * *z) * dt;
    *x += dx;
    *y += dy;
    *z += dz;
}
