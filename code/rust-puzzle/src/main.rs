#[derive(Debug)]

/*泛型结构体
使用泛型的目的，在于将for循环中的循环次数i作为参数传入pieces_index,原本设计pieces_index是u32类型，
但是在直接将i赋值给pieces_index的时候发现i的数据类型为未知数据长度(usize)，数据类型不匹配导致不能直接
赋值，还没搞懂为什么会是usize，先用泛型解决
*/
struct encode_packet<T> {   
    pieces_count:T,
    pieces_index:T,
    pieces_data: String,
}

fn main() {
    let base_data = String::from("12345678223456783234567842345678");
    println!("base data: {}",base_data);
    let encode_data = puzzle_encode(base_data); //base_data moved,can't be borrowed at line 19

    println!("encode data: {}",encode_data);
    //println!("base data: {}",base_data);
}

fn puzzle_encode(input_data: String) -> String{
    let out_data = input_data;

    let input_len = out_data.len();
    let pieces_count = (input_len+7)/8;
    println!("{} data separate to {} pieces!",input_len,pieces_count);

    for i in 0..pieces_count{
        let picecs_data = &out_data[i*8..(i+1)*8];
        let test_pac = encode_packet{
            pieces_count : 4,
            pieces_index : i,
            pieces_data : String::from(picecs_data),
        };
        println!("the {} pieces is {:#?}", i,test_pac);
    }
    out_data
}