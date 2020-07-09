use triton::{Error, FutharkContext};

pub fn main() -> Result<(), Error> {
    let n = 5i32;
    let (vec, chunk_size) = simple_8(n)?;

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

fn simple_8(n: i32) -> Result<(Vec<u64>, usize), Error> {
    let mut ctx = FutharkContext::new();

    let res_arr = ctx.simple8(n)?;
    let (vec, shape) = res_arr.to_vec();
    assert_eq!(n as i64, shape[0]);
    let chunk_size = shape[1] as usize;

    assert_eq!(2, shape.len());
    Ok((vec.to_vec(), chunk_size))
}

#[test]
fn test_results() {
    let n = 5;
    let (vec, _) = simple_8(n).unwrap();

    let expected = vec![
        1268321962943061524,
        14734993503448812560,
        18034603940993995658,
        3104019895137717477,
        10995972505292001199,
        17865534512424957083,
        8587190255961617306,
        7897662749342968582,
        16178010229119263899,
        13010854163453830554,
        10439949954613982629,
        2530811627786879353,
        12770435378198366767,
        12273694882464091504,
        11099785331338426777,
        6041376865343808761,
        9928799194434130384,
        7506436120463506388,
        8762943653766285922,
        1919693068824444217,
    ];

    assert_eq!(expected, vec);
}
