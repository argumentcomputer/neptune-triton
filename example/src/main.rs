use triton::{Error, FutharkContext};

pub fn main() -> Result<(), Error> {
    let mut ctx = FutharkContext::new();

    let res_arr = ctx.simple8(5)?;
    let (vec, shape) = &res_arr.to_vec();
    let n = shape[0];
    let chunk_size = shape[1] as usize;

    assert_eq!(2, shape.len());

    for (i, chunk) in vec.chunks(chunk_size).enumerate() {
        print!("res {} of {}: ", i, n);
        print!("[");
        for elt in chunk.iter() {
            print!("{}, ", elt);
        }
        println!("]");
    }

    Ok(())
}
