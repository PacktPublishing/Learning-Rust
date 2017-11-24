fn main() 
{
    // first create a couple of arrays - these will be used
    // for the vectors
    let line1: [i32; 4] = [4, 2, 3, 3];
    let line2: [i32; 4] = [3, 4, 5, 7];
    let line3: [i32; 4] = [2, 9, 6, 2];
    let line4: [i32; 4] = [5, 7, 2, 4];

    // create two holding arrays and assign
    // we are creating an array of references
    let array_one = [&line1, &line3, &line4, &line2];
    let array_two = [&line2, &line1, &line3, &line4];

    // let's do the multiply
    // we are passing in a ref array containing ref arrays
    let result = matrix_multiply(&array_one, &array_two);
    println!("{:?}", result);
}

fn matrix_multiply(vec1: &[&[i32;4];4], vec2: &[&[i32;4];4]) -> [[i32; 4];4]
{
    // we need to create the arrays to put the results into
    let mut result = [[0i32; 4]; 4];

    // loop through the two vectors
    for vone in 0..4
    {
       for vtwo in 0..4
       {
          let mut sum = 0;
          for k in 0..4
          {
               sum += vec1[vone][k] * vec2[k][vtwo];
          }
          result[vone][vtwo] = sum;
       }
    }
    result
}
