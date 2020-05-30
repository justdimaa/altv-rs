// #[derive(Serialize)]
// pub struct Turf {
//     x1: f32,
//     y1: f32,
//     x2: f32,
//     y2: f32,
// }
//
// impl Turf {
//     pub fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Turf {
//         Turf { x1, y1, x2, y2 }
//     }
//
//     pub fn between(&self, min: f32, p: f32, max: f32) -> bool {
//         let mut result = false;
//
//         if min < max {
//             if p > min && p < max {
//                 result = true;
//             }
//         }
//
//         if min > max {
//             if p > max && p < min {
//                 result = true;
//             }
//         }
//
//         if p == min || p == max {
//             result = true
//         }
//
//         result
//     }
//
//     pub fn contains(&self, x: f32, y: f32) -> bool {
//         self.between(self.x1, x, self.x2) && self.between(self.y1, y, self.y2)
//     }
//
//     pub fn get_x1(&self) -> f32 {
//         self.x1
//     }
//
//     pub fn get_y1(&self) -> f32 {
//         self.y1
//     }
//
//     pub fn get_x2(&self) -> f32 {
//         self.x2
//     }
//
//     pub fn get_y2(&self) -> f32 {
//         self.y2
//     }
// }
