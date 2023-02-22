use proconio::input;
use std::num::ParseIntError;

struct NamedVector {
  name:String,
  vector:Vec<i32>,
}

fn SumVector(vector1:&Vec<i32>, vector2:&Vec<i32>) -> Vec<i32>{
  if vector1.len() != vector2.len() { 
    return vec![0; 8];
  }
  let mut vec_sum: Vec<i32> = vec![0; vector1.len()];
  for i in 0..vector1.len(){
    vec_sum[i] = vector1[i] + vector2[i];
  }
  return vec_sum;
}

fn TruncateVectorElement(vector:&Vec<i32>, min:i32, max:i32) -> Vec<i32>{
  let mut vec_trimed: Vec<i32> = Vec::new();
  for i in 0..vector.len(){
    let element = vector[i];
    vec_trimed.push(
      if      element < min {min}
      else if element > max {max}
      else                  {element}
    )
  }
  return vec_trimed;
}

fn main() {
  const kNumFragment: usize = 11;
  let star_fragments: [NamedVector; kNumFragment] = [
    //                  名称                                     力,  守備,  技, 速さ,幸運,魔防,  HP, 武器
    NamedVector { name: "Aquarius".to_string()   , vector: vec![ 10,    0,  10,  10,   0,   0,   0,  10] },
    NamedVector { name: "Pisces".to_string()     , vector: vec![  0,   10,   0,   0,  10,  10,  10,   0] },
    NamedVector { name: "Aries".to_string()      , vector: vec![  0,    0,   0,   0,  40,   0,   0,   0] },
    NamedVector { name: "Taurus".to_string()     , vector: vec![  5,    5,   5,   5,   5,   5,   5,   5] },
    NamedVector { name: "Gemini".to_string()     , vector: vec![ 30,   20,   0,   0,   0,   0,   0, -10] },
    NamedVector { name: "Cancer".to_string()     , vector: vec![-10,   50,   0,   0,   0,   0,   0,   0] },
    NamedVector { name: "Leo".to_string()        , vector: vec![ 50,  -10,   0,   0,   0,   0,   0,   0] },
    NamedVector { name: "Virgo".to_string()      , vector: vec![  0,  -10,   0,   0,   0,  30,   0,  20] },
    NamedVector { name: "Libra".to_string()      , vector: vec![  0,    0,   0,  40,  10, -10, -10,  10] },
    NamedVector { name: "Scorpio".to_string()    , vector: vec![ 20,    0,  20,  10, -10,   0,   0,   0] },
    NamedVector { name: "Sagittarius".to_string(), vector: vec![  0,    0,  40,  10,   0,   0, -10,   0] },
  ];

  input! {
    unit_name: String,
  }

  const kNumStatus: usize = 8;

  for i1 in 0..kNumFragment-3{
    for i2 in i1+1..kNumFragment-2{
      for i3 in i2+1..kNumFragment-1{
        for i4 in i3+1..kNumFragment-0{
          let mut growth_vec_sum: Vec<i32> = vec![0; kNumStatus];
          growth_vec_sum = SumVector(&growth_vec_sum, &star_fragments[i1].vector);
          growth_vec_sum = SumVector(&growth_vec_sum, &star_fragments[i2].vector);
          growth_vec_sum = SumVector(&growth_vec_sum, &star_fragments[i3].vector);
          growth_vec_sum = SumVector(&growth_vec_sum, &star_fragments[i4].vector);
          
          let growth_vec_sum_trimed = TruncateVectorElement(&growth_vec_sum, 0, 100);
          println!("{:?}", growth_vec_sum_trimed);
        }
      }
    }
  }
}