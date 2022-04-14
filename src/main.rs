use std::num::Wrapping;

fn main() {
    // 第一题
    let traffic_light = TrafficLight::Red;
    let time = traffic_light.get_light_time();
    println!("the light time is {} s", time);

    // 第二题
    let u32_set = [1u32, u32::MAX];
    let rst = u32_sum(&u32_set);
    match rst {
        Some(t) => println!("{}", t),
        None => println!("数值溢出")
    }

    // 第三题
    let square = Geometry{
        l: 4.0,
        h: 0.0,
        t: GeometryType::Square
    };
    get_area(square);
}

// 定义trait
trait LightTime {
    fn get_light_time(&self) -> i32;
}

// 定义交通灯
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

// 交通灯实现
impl LightTime for TrafficLight {
    fn get_light_time(&self) -> i32 {
        match &self {
            TrafficLight::Red => 60,
            TrafficLight::Green => 90,
            TrafficLight::Yellow => 10
        }
    }
}

// 2. 为u32集合做加法
fn u32_sum(uset: &[u32]) -> Option<u32> {
    let mut r = 0;
    for i in uset {
        if u32::MAX - *i > r {
            r += i
        } else {
            return None;
        }
    }
    Some(r)
}

// 3.打印图形面积的函数
enum GeometryType {
    Square,
    Round,
    Triangle
}

struct  Geometry
{
    l: f32,
    h: f32,
    t: GeometryType
}

trait ComputeArea {
    fn area(&self) -> f32;
}

impl ComputeArea for Geometry
{
    fn area(&self) -> f32 {
        match self.t {
            GeometryType::Square => self.l * self.l,
            GeometryType::Round => self.l * self.l * 3.14,
            GeometryType::Triangle => self.l * self.h * 0.5,
        }
    }
}


fn get_area<T: ComputeArea>(geometry: T)
{
    let current_geo_area = geometry.area();
    println!("{}", current_geo_area)
}


