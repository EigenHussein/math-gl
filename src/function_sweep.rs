use std::marker::PhantomData;

use nalgebra_glm::Vec4;

pub fn sweep<'a, T>(a: &'a T, b: &'a T, t: f32) -> T
where
    &'a T: std::ops::Add<&'a T, Output = T>
        + std::ops::Sub<&'a T, Output = T>
        + std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>,
    // + std::ops::Mul<f32, Output = &'a T>,
    T: std::ops::Add<&'a T, Output = T>
        + std::ops::Sub<&'a T, Output = T>
        + std::ops::Mul<f32, Output = T>,
{
    a + (b - a) * t
}
pub fn sweep_vec4(a: &Vec4, b: &Vec4, t: f32) -> Vec4 {
    a + (b - a) * t
}


pub struct PiecewiseFunction<A, B> {
    function: Box<dyn Fn(A) -> B>,
    start_time: f32,
    stop_time: f32,
}

impl<A, B> PiecewiseFunction<A, B> {
    pub fn new(function: Box<dyn Fn(A) -> B>, start: f32, stop: f32) -> Self {
        Self {
            function: function,
            start_time: start,
            stop_time: stop,
        }
    }
}

impl<B> PiecewiseFunction<f32, B> {
    pub fn eval(&self, t: f32) -> B {
        (self.function)(t /* - self.start_time */)
    }
    pub fn in_range(&self, t: f32) -> bool {
        t >= self.start_time && t < self.stop_time
    }
}

pub struct FunctionCollection<A, B> {
    functions: Vec<(PiecewiseFunction<A, B>, Box<dyn Fn(f32)->f32>)>, // Function + Activation
}

impl<A, B> FunctionCollection<A, B> {
    pub fn new(function: Box<dyn Fn(A) -> B>, stop: f32, activation:Option<Box<dyn Fn(f32)->f32>>) -> Self {
        //PiecewiseFunction::new(function, start, stop);
        Self {
            functions: vec![(PiecewiseFunction::new(function, 0.0, stop), activation.unwrap_or(Box::new(|t|t)))],
        }
    }
    pub fn add_function(
        &mut self,
        function: Box<dyn Fn(A) -> B>,
        duration: f32,
        activation:Option<Box<dyn Fn(f32)->f32>>,
    ) -> &mut Self {
        let stop_time = self.functions.last().unwrap().0.stop_time;
        self.functions.push((
            PiecewiseFunction::new(function, stop_time, stop_time + duration),
            activation.unwrap_or(Box::new(|t|t)),
        ));
        self
    }
}
impl FunctionCollection<f32, Vec4> {
    pub fn eval(&self,sweep:f32, t: f32) -> Vec4 {
        let n_functions = self.functions.len();
        let mut res = 0.0f32;
        for i in 0..n_functions - 1 {
            let current_piece = &self.functions[i];
            let mid_time = (current_piece.0.stop_time + current_piece.0.start_time) / 2.0;
            if current_piece.0.in_range(sweep) {
                if sweep <= mid_time {
                    return current_piece.0.eval(t);
                } else {
                    let f_left = current_piece.0.eval(t);
                    let f_right = self.functions[i + 1].0.eval(t);
                    return sweep_vec4(
                        &f_left,
                        &f_right,
                    (self.functions[i].1) ((sweep - mid_time)
                            / (current_piece.0.stop_time-mid_time)),
                    );
                }
            }
        }
        return self.functions.last().unwrap().0.eval(t);
    }
}


// pub fn sweep_direct<T>(a: &T, b: &T,t:f32) -> T
// where
//     T: std::ops::Add<T, Output = T> + std::ops::Sub<T, Output = T> + std::ops::Mul<f32, Output = T>,
// {

//     a + (b - &a) * t
// }
// pub fn ez_sweep<'a, T>(a: &T, b: &T, t: f32) -> T
// where
//     T: std::ops::Add<&'a T, Output = T>
//         + std::ops::Sub<&'a T, Output = T>
//         + std::ops::Mul<f32, Output = T>
//         + 'a,
// {
//     let tmp =b-a;

//     a + (b - a) * t
// }
// pub fn ez_sweep2<T>(a: T, b: T, t: f32) -> T
// where
//     T: Clone
//         + Default
//         + std::ops::Add<T, Output = T>
//         + std::ops::Sub<T, Output = T>
//         + std::ops::Mul<f32, Output = T>,
// {
//     a + (b - a) * t
// }

// #[derive(Default)]
// pub struct ConstSweep<T>
// where
//     T: Clone
//         + Default
//         + std::ops::Add<T, Output = T>
//         + std::ops::Sub<T, Output = T>
//         + std::ops::Mul<f32, Output = T>,
// {
//     _a: PhantomData<T>,
//     values: Vec<(T, f32, f32, f32)>, //struct,start_time, stop_time, speed
//     current_value_index: usize,
// }

// fn sigmoid(t: f32) -> f32 {
//     return 1.0 / (1.0 + f32::exp(t));
// }

// impl<T> ConstSweep<T>
// where
//     T: Clone
//         + Default
//         + std::ops::Add<T, Output = T>
//         + std::ops::Sub<T, Output = T>
//         + std::ops::Mul<f32, Output = T>,
// {
//     pub fn add_element(&mut self, element: T, duration: f32, transition_time: f32) -> &mut Self {
//         if self.values.len() == 0 {
//             self.values.push((element, 0.0, duration, transition_time));
//         } else {
//             let previous_stop_time = self.values.last().unwrap().2;
//             self.values.push((
//                 element,
//                 previous_stop_time,
//                 previous_stop_time + duration,
//                 transition_time,
//             ));
//         }
//         self
//     }
//     pub fn eval_random(&mut self, t: f32) -> T {
//         // let mut res: T = Default::default();
//         let nPieces = self.values.len();
//         for i in 0..nPieces {
//             let current_piece = &self.values[i];
//             let start_time = current_piece.1;
//             let stop_time = current_piece.2;

//             if t >= start_time && t < stop_time {
//                 let mid_time = (start_time + stop_time) / 2.0;
//                 if t < mid_time {
//                     return current_piece.0.clone();
//                 } else {
//                     if i < nPieces - 1 {
//                         ez_sweep2(10.0, 20.0, 1.0);

//                         return ez_sweep2(
//                             self.values[i].0.clone(),
//                             self.values[i + 1].0.clone(),
//                             (t - mid_time) / (stop_time - mid_time),
//                         );
//                     } else {
//                         return self.values[i].0.clone();
//                     }
//                 }
//             }
//         }
//         Default::default()
//     }
// }

// #[derive(Default)]
// pub struct ConstSweep<T>
// where
//     T: Clone
//         + Default
//         + std::ops::Add<T, Output = T>
//         + std::ops::Sub<T, Output = T>
//         + std::ops::Mul<f32, Output = T>,
// {
//     _a: PhantomData<T>,
//     values: Vec<(T, f32, f32, f32)>, //struct,start_time, stop_time, speed
//     current_value_index: usize,
// }

// fn sigmoid(t: f32) -> f32 {
//     return 1.0 / (1.0 + f32::exp(t));
// }

// impl<T> ConstSweep<T>
// where
//     T: Clone
//         + Default
//         + std::ops::Add<T, Output = T>
//         + std::ops::Sub<T, Output = T>
//         + std::ops::Mul<f32, Output = T>,
// {
//     pub fn add_element(&mut self, element: T, duration: f32, transition_time: f32) -> &mut Self {
//         if self.values.len() == 0 {
//             self.values.push((element, 0.0, duration, transition_time));
//         } else {
//             let previous_stop_time = self.values.last().unwrap().2;
//             self.values.push((
//                 element,
//                 previous_stop_time,
//                 previous_stop_time + duration,
//                 transition_time,
//             ));
//         }
//         self
//     }
//     pub fn eval_random(&mut self, t: f32) -> T {
//         // let mut res: T = Default::default();
//         let nPieces = self.values.len();
//         for i in 0..nPieces {
//             let current_piece = &self.values[i];
//             let start_time = current_piece.1;
//             let stop_time = current_piece.2;

//             if t >= start_time && t < stop_time {
//                 let mid_time = (start_time + stop_time) / 2.0;
//                 if t < mid_time {
//                     return current_piece.0.clone();
//                 } else {
//                     if i < nPieces - 1 {
//                         ez_sweep2(10.0, 20.0, 1.0);

//                         return ez_sweep2(
//                             self.values[i].0.clone(),
//                             self.values[i + 1].0.clone(),
//                             (t - mid_time) / (stop_time - mid_time),
//                         );
//                     } else {
//                         return self.values[i].0.clone();
//                     }
//                 }
//             }
//         }
//         Default::default()
//     }
// }
