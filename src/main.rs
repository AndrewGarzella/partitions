use std::time::Instant;

fn main() {    
    let before = Instant::now();
    println!("{} partitions in 100", partitions_for_num(100, true));
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn partitions_for_num(num: usize, print_solutions: bool) -> usize {
    if print_solutions {
        if num == 0 {
            println!("There are no partions for zero");
            return 0;
        }
        if num == 1 { 
            print!("Solution 1: 1 ");
            return 1;
        }
        if num == 2 { 
            println!("Solution 1: 1 1\nSolution 2: 2 0");
            return 2;
        }

        let mut parts: Vec<usize> = vec![0;num];
        parts[0] = num;

        let mut solutions: Vec<Vec<usize>> = Vec::new();

        solutions.push(parts.clone());

        loop {

            // if first number in parts is a 1 then pushes to solutions and end loop
            if parts[0] == 1 {
                break;
            }

            let mut needs_compresion: bool = false;
        
            
            let mut compress_at = 1;
            for _i in 0..parts.len() {
                if parts[_i] == 1 {
                    compress_at = _i - 1;
                    needs_compresion = true;
                    break;
                }
            }
            
            if needs_compresion  {
                let mut prev = 0;
                for _i in 0..compress_at {
                    prev = prev + parts[_i];
                }

                parts = compress(parts, compress_at, num - prev);

            } else {
                for _i in (0..parts.len()).rev() {
                    if parts[_i-1] > parts[_i] && parts[_i-1] - 1 != 0 {    
                        parts[_i] += 1;                    
                        parts[_i-1] -= 1;
                        break;      
                    } 
                }
            }
            
            //checks if parts adds up to a solutions
            let mut temp = 0;
            for _i in 0..parts.len()  {
                temp = temp + parts[_i];
            }

            //adds to solutions 
            if temp == num{
                solutions.push(parts.clone());
            }
        }

        // prints soluionts 
        for _i in 0..solutions.len() {
            print!("Solution {}: ", _i + 1);
            for _j in 0..solutions[_i].len() {
                print!("{} ", solutions[_i][_j]);
            }
            println!("");
        }
        return solutions.len();
    } else {
        if num == 0 {
            return 0;
        }
        if num == 1 { 
            return 1;
        }
        if num == 2 { 
            return 2;
        }

        let mut parts: Vec<usize> = vec![0;num];
        parts[0] = num;
        let mut solutions: usize = 0;

        solutions += 1;

        loop {

            // if first number in parts is a 1 then pushes to solutions and end loop
            if parts[0] == 1 {
                break;
            }

            let mut needs_compresion: bool = false;
        
            
            let mut compress_at = 1;
            for _i in 0..parts.len() {
                if parts[_i] == 1 {
                    compress_at = _i - 1;
                    needs_compresion = true;
                    break;
                }
            }
            
            if needs_compresion  {
                let mut prev = 0;
                for _i in 0..compress_at {
                    prev = prev + parts[_i];
                }

                parts = compress(parts, compress_at, num - prev);

            } else {
                for _i in (0..parts.len()).rev() {
                    if parts[_i-1] > parts[_i] && parts[_i-1] - 1 != 0 {    
                        parts[_i] += 1;                    
                        parts[_i-1] -= 1;
                        break;      
                    } 
                }
            }
            //adds to solutions 
            solutions += 1;
        }

        return solutions;

    }

}

fn compress(list: Vec<usize>,start: usize, total: usize) -> Vec<usize> {
    let mut compressed_list = list.clone();

    // Senerio 1 
    if list[start] - 1 > total - list[start] {
        compressed_list[start] = list[start] - 1;
        compressed_list[start+1] = total-list[start]+1;
        for _i in start+2..list.len() {
            compressed_list[_i] = 0;
        }
    }  else { // Senerio 2
        if list[start] == 2 {
            compressed_list[start] -= 1;
            for _i in (start..list.len()-1).rev() {
                if list[_i] == 1 {
                    compressed_list[_i + 1] += 1;
                    return compressed_list;
                }
            }
        }
        let x = total / (list[start] - 1);
        let y = total % (list[start] - 1);
        for _i in start..x+start {
            compressed_list[_i] = list[start] - 1;
        }
        compressed_list[x + start] = y;
        for _i in x+start+1..list.len() {
            compressed_list[_i] = 0;
        }
    }

    return compressed_list;
}
