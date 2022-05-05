fn solution(inputArray: Vec<i32>) -> i32 {
 //largest step 
 let mut max = inputArray.iter().max().unwrap() + 1;
  
 for n in 2..max {
   //
   let result = inputArray.iter().map(|x| x % n).min().unwrap();
   
   if 0 != result {
     return n;
   } 
 }
 max
}
  

fn main() {
    let inputArray = vec![5, 3, 6, 7, 9];
    let mut jump = 0;
    for n in 2.. {
        let result = inputArray.iter().map(|x| x % n).min().unwrap();
        println!("{}-{:?}-{:?}",n, inputArray.iter().map(|&x| x % n).collect::<Vec<i32>>(), inputArray.iter().map(|x| x % n).min() );
        if 0 != result {
            jump = n;
            break;
        }
    }
}


#[cfg(test)]
mod test {
    use crate::solution;
    #[test]
    fn test1() {
        let test1 = vec![5, 3, 6, 7, 9];
        let expected = solution(test1);
        let result = 4;
        assert_eq!(&result, &expected, "Fail")
    }
}