
// x:2,--------&x:0x16b6ae818
// y:3,--------&y:0x16b6ae81c
// z:2,--------&z:0x16b6ae918
// z:3,--------&z:0x16b6ae918
// z:3,--------&z:0x16b6ae918
// x:3,--------&x:0x16b6ae818
// y:3,--------&y:0x16b6ae81c
fn main() {
    let mut x = 2;
    let mut y = 3;

    println!("x:{},--------&x:{:p}",x,&x);
    println!("y:{},--------&y:{:p}",y,&y);

    {
        let mut z = &mut x;
        println!("z:{},--------&z:{:p}",z,&z);

        *z += 1;

        println!("z:{},--------&z:{:p}",z,&z);

        z = &mut y;
        println!("z:{},--------&z:{:p}",z,&z);
    }

    println!("x:{},--------&x:{:p}",x,&x);
    println!("y:{},--------&y:{:p}",y,&y);
}