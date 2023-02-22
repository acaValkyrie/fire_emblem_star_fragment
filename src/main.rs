use proconio::input;

struct NamedVector {
  name:String,
  vector:Vec<i32>,
}

fn SumVector(vector1:Vec<i32>, vector2:Vec<i32>) -> Vec<i32>{
  if vector1.len() != vector2.len() { 
    return vector1; 
  }
  let vec_sum: Vec<i32>;
  for i in 0..vector1.len(){
    vec_sum[i] = vector1[i] + vector2[i];
  }
  return vec_sum;
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

  for i1 in 0..kNumFragment-3{
    for i2 in i1+1..kNumFragment-2{
      for i3 in i2+1..kNumFragment-1{
        for i4 in i3+1..kNumFragment-0{
          let growth_rate_vec: Vec<i32>;
          
        }
      }
    }
  }
}