// Max Scott
// updated: 5/7 2024
// a bunch of linar algebra functions (to be made into a cargo library at some point)


// test 
fn main() {
    
    let a = vec![
        vec![1i32, 2, 3],
        vec![1i32, 2, 3],
        vec![1i32, 2, 3]
    ];

    let b = vec![
        vec![8i32, 3, 6],
        vec![8i32, 3, 6],
        vec![8i32, 7, 6]
    ];

    print_vec(b[0].clone());
    print_mat(b.clone());
    println!("");


    mat_mult(a.clone(),b.clone());



}

// print vec funk 
fn print_vec(vec: Vec<i32>){
    for i in 0..vec.len(){
        print!("{}, ", vec[i]);
    }
}

// print mat 

fn print_mat(mat: Vec<Vec<i32>>){
    for r in 0..mat[0].len(){
        println!("");
        for c in 0..mat.len(){
            print!("{}, ", mat[r][c])
        }
    }
}


// make dot prod function 
fn dot_prod(a: Vec<i32>, b: Vec<i32>) -> i32{
    // a1 * b1 + a2 * b2 ... 
    let mut sum: i32 = 0;
    for i in 0..a.len(){
        sum = sum + (a[i] * b[i]);
    }
    sum
}

// make mat mult function 
fn mat_mult(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>>{
    for r in 0..b[0].len(){
        // get col
        let mut col: Vec<i32> = Vec::new();
        for c in 0..a.len(){
            col.push(a[r][c]);
        }
        let dot = dot_prod(col, b[r].clone());
        println!("{}", dot);
    }
    a.clone()
}


// inverce funk 


// transpose funk


// row eshilon funk 


// det funk 


// is liniar combo funk 




