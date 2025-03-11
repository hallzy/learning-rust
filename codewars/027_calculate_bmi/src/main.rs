fn bmi(weight: u32, height: f32) -> &'static str {
    match (weight as f32) / (height * height) {
        n if n <= 18.5 => "Underweight",
        n if n <= 25.0 => "Normal",
        n if n <= 30.0 => "Overweight",
        _ => "Obese",
    }
}

fn main() {
    assert_eq!(bmi(50, 1.80), "Underweight");
    assert_eq!(bmi(80, 1.80), "Normal");
    assert_eq!(bmi(90, 1.80), "Overweight");
    assert_eq!(bmi(110, 1.80), "Obese");
}
