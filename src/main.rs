use proconio::input;

struct NamedVector {
  name:String,
  vector:Vec<i32>,
}

fn AddVector(vector1:&Vec<i32>, vector2:&Vec<i32>) -> Vec<i32>{
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

fn FindUnitVector(unit_name: String, unit_list: &[NamedVector]) -> Vec<i32> {
  for i in 0..unit_list.len() {
    if unit_list[i].name == unit_name { 
      return unit_list[i].vector.clone();
    }
  }
  println!("指定された名前 : {} のユニットが見つかりませんでした。", unit_name);
  return vec![0];
}

fn SumVector(vec: &Vec<i32>) -> i32{
  let mut sum: i32 = 0;
  for i in 0..vec.len() {
    sum += vec[i];
  }
  sum
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
    NamedVector { name: "Sagittarius".to_string(), vector: vec![  0,    0,  40,  10,   0,   0, -10,   0] },];

  const kNumUnit: usize = 45;
  let units: [NamedVector; kNumUnit] = [
    NamedVector {name: "マルス".to_string()    , vector: vec![ 50, 20, 40, 50, 70, 3, 90, 40] },
    NamedVector {name: "アラン".to_string()    , vector: vec![ 10, 10, 10, 10,  0, 3, 10, 10] },
    NamedVector {name: "ルーク".to_string()    , vector: vec![ 30, 20, 30, 40, 10, 3, 90, 40] },
    NamedVector {name: "ロディ".to_string()    , vector: vec![ 40, 20, 50, 50, 40, 3, 70, 60] },
    NamedVector {name: "セシル".to_string()    , vector: vec![ 30, 10, 60, 60, 30, 3, 50, 70] },
    NamedVector {name: "ドーガ".to_string()    , vector: vec![ 20, 10, 40, 40, 20, 3, 60, 40] },
    NamedVector {name: "ゴードン".to_string()  , vector: vec![ 30, 10, 30, 30, 40, 3, 40, 60] },
    NamedVector {name: "ライアン".to_string()  , vector: vec![ 40, 20, 70, 40, 30, 3, 60, 40] },
    NamedVector {name: "マリーシア".to_string(), vector: vec![ 30, 10, 20, 50, 30, 3, 40, 70] },
    NamedVector {name: "カチュア".to_string()  , vector: vec![ 40, 20, 80, 80, 40, 3, 70, 70] },
    NamedVector {name: "ウォレン".to_string()  , vector: vec![ 50, 20, 50, 10, 20, 3, 70, 20] },
    NamedVector {name: "リンダ".to_string()    , vector: vec![ 20, 10, 70, 60, 80, 3, 40, 70] },
    NamedVector {name: "パオラ".to_string()    , vector: vec![ 50, 20, 80, 20, 10, 3, 70, 30] },
    NamedVector {name: "ジュリアン".to_string(), vector: vec![ 70, 30, 50, 50, 80, 3, 80, 40] },
    NamedVector {name: "マチス".to_string()    , vector: vec![ 40, 20, 30, 20, 20, 3, 50, 70] },
    NamedVector {name: "オグマ".to_string()    , vector: vec![ 40, 30, 30, 30, 40, 3, 80, 70] },
    NamedVector {name: "ユミナ".to_string()    , vector: vec![ 20, 10, 30, 40, 40, 3, 30, 60] },
    NamedVector {name: "ユベロ".to_string()    , vector: vec![ 50, 10, 30, 40, 40, 3, 50, 60] },
    NamedVector {name: "シリウス".to_string()  , vector: vec![ 50, 30, 50, 40, 30, 3, 80, 40] },
    NamedVector {name: "カシム".to_string()    , vector: vec![ 60, 20, 40, 40, 20, 3, 70, 20] },
    NamedVector {name: "シーダ".to_string()    , vector: vec![ 20, 20, 70, 90, 70, 3, 50, 80] },
    NamedVector {name: "リカード".to_string()  , vector: vec![ 50, 20, 20, 60, 40, 3, 50, 30] },
    NamedVector {name: "サムトー".to_string()  , vector: vec![ 20, 10, 20, 60, 10, 3, 70, 40] },
    NamedVector {name: "ウェンデル".to_string(), vector: vec![ 10, 10, 30, 20, 40, 3, 60, 70] },
    NamedVector {name: "ナバール".to_string()  , vector: vec![ 50, 20, 40, 50, 60, 3, 90, 30] },
    NamedVector {name: "フィーナ".to_string()  , vector: vec![ 60, 20, 70, 80, 70, 3, 40, 80] },
    NamedVector {name: "カイン".to_string()    , vector: vec![ 30, 20, 60, 60, 50, 3, 90, 60] },
    NamedVector {name: "バヌトゥ".to_string()  , vector: vec![  0,  0,  0,  0, 10, 3, 10,  0] },
    NamedVector {name: "ジョルジュ".to_string(), vector: vec![ 20, 10, 10, 20, 10, 3, 60, 50] },
    NamedVector {name: "ミネルバ".to_string()  , vector: vec![ 30, 20, 50, 40, 40, 3, 40, 70] },
    NamedVector {name: "マリク".to_string()    , vector: vec![ 20, 20, 50, 50, 50, 3, 80, 80] },
    NamedVector {name: "エルレーン".to_string(), vector: vec![ 10, 30, 30, 40, 10, 3, 90, 70] },
    NamedVector {name: "チェイニー".to_string(), vector: vec![ 30, 20, 10, 40, 30, 3, 50, 30] },
    NamedVector {name: "チキ".to_string()      , vector: vec![ 40,  0, 30, 60, 60, 3, 80,  0] },
    NamedVector {name: "エスト".to_string()    , vector: vec![ 70, 20, 70, 70, 60, 3, 50, 70] },
    NamedVector {name: "アベル".to_string()    , vector: vec![ 40, 20, 50, 50, 40, 3, 70, 70] },
    NamedVector {name: "アストリア".to_string(), vector: vec![ 50, 10, 40, 20, 50, 3, 90, 40] },
    NamedVector {name: "シーマ".to_string()    , vector: vec![ 60, 20, 70, 50, 60, 3, 60, 60] },
    NamedVector {name: "サムソン".to_string()  , vector: vec![ 30, 20, 10, 20, 50, 3, 70, 70] },
    NamedVector {name: "ロシェ".to_string()    , vector: vec![ 40, 30, 50, 40, 50, 3, 80, 30] },
    NamedVector {name: "ミディア".to_string()  , vector: vec![ 30, 20, 50, 50, 10, 3, 80, 50] },
    NamedVector {name: "レナ".to_string()      , vector: vec![ 10, 10, 10, 20, 40, 3, 10, 30] },
    NamedVector {name: "マリア".to_string()    , vector: vec![ 10, 10, 10, 20, 30, 3, 10, 70] },
    NamedVector {name: "ニーナ".to_string()    , vector: vec![ 10, 10, 10, 10, 10, 3, 10, 30] },
    NamedVector {name: "エリス".to_string()    , vector: vec![ 10, 10, 50, 60, 80, 3, 10, 90] }];

  input! {
    unit_name: String,
  }

  let basic_growth: Vec<i32> = FindUnitVector(unit_name, &units);
  let mut max_growth = 0;
  let mut fragment_num: [usize; 4] = [0; 4];
  let mut max_growth_vector: Vec<i32> = Vec::new();

  for i1 in 0..kNumFragment-3{
    for i2 in i1+1..kNumFragment-2{
      for i3 in i2+1..kNumFragment-1{
        for i4 in i3+1..kNumFragment-0{
          let mut growth_vec_sum: Vec<i32> = basic_growth.clone();
          growth_vec_sum = AddVector(&growth_vec_sum, &star_fragments[i1].vector);
          growth_vec_sum = AddVector(&growth_vec_sum, &star_fragments[i2].vector);
          growth_vec_sum = AddVector(&growth_vec_sum, &star_fragments[i3].vector);
          growth_vec_sum = AddVector(&growth_vec_sum, &star_fragments[i4].vector);
          
          let growth_vec_sum_trimed = TruncateVectorElement(&growth_vec_sum, 0, 100);
          let growth_sum = SumVector(&growth_vec_sum_trimed);
          if growth_sum > max_growth {
            max_growth = growth_sum;
            fragment_num = [i1, i2, i3, i4];
            max_growth_vector = growth_vec_sum_trimed;
          }
        }
      }
    }
  }
  for i in 0..4{
    println!("{}", star_fragments[ fragment_num[i] ].name);
  }
  println!("{:?}", max_growth_vector);
}