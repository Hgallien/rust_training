// extern crate matrix;
use rand::{Rng,thread_rng};
// use generic_matrix::Matrix;
// extern crate matrix;
use nalgebra::DMatrix;
// use matrix::prelude::*;
struct Taquin{
    size : i32,
    matrix : DMatrix::<i32>,
    heuri: i32,
}
fn random_martrix(x : usize) ->  DMatrix<i32>
{
    let mut matrix :DMatrix<i32>  = DMatrix::<i32>::zeros(x,x);

    for i in 1..(x*x)
    {
        let distance_to_spot: usize = thread_rng().gen_range(1..((x*x) - (i-1)) as i32) as usize;
        //final place in matrix
        let mut fp : usize=0 ;
        for _y in 0..distance_to_spot 
        {
            let mut p1 = (fp ) / (x);
            let mut p2 = (fp ) % (x);
            while matrix[(p1 ,p2)]!= 0
            {
                fp+=1;
                 p1 = fp / (x);
                 p2 = fp % (x);
            }
            fp+=1;
        }
        matrix[((fp - 1)/ x,(fp - 1) %x)] = i as i32;
    }
    return matrix;
}

fn random_martrix_v2(x : usize) ->  DMatrix<i32>
{
    let mut matrix :DMatrix<i32>  = DMatrix::<i32>::zeros(x,x);
    let mut vec : Vec<i32> = (0..((x*x) as i32)).collect();
    for i in 1..(x*x)
    {
        let distance_to_spot: usize = thread_rng().gen_range(0..((x*x) - (i-1)) as i32) as usize;
        let  test = matrix.get_mut(((vec[distance_to_spot] as usize) / x, (vec[distance_to_spot] as usize) % x));
        match test {
              None => (),
              Some(t) => *t = i as i32,
        }
        vec.remove(distance_to_spot);
    }
      return matrix;
}
fn main(){
    let matrix = random_martrix(4);
    println!(" {}",matrix);
    let matrix = random_martrix_v2(3);
    println!(" {}",matrix);
}
