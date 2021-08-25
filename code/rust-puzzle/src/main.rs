use std::io;
//用于捕捉keyboard输入
use std::cmp::Ordering;
//原本想用match的方式来对数据包是否为最后一包进行判断，但是遇到变量作用域的问题未解决，此处先不用，待掌握作用域后再做优化

/*泛型结构体
使用泛型的目的，在于将for循环中的循环次数i作为参数传入pieces_index,原本设计pieces_index是u32类型，
但是在直接将i赋值给pieces_index的时候发现i的数据类型为未知数据长度(usize)，数据类型不匹配导致不能直接
赋值，还没搞懂为什么会是usize，先用泛型解决
*/
#[derive(Debug)]
struct encode_packet<T> {   
    pieces_count:T,
    pieces_index:T,
    pieces_data: String,
}

fn main() {
    let mut base_data = String::new();

    println!("input your base date!");
    io::stdin().read_line(&mut base_data).expect("failed to read line");
    println!("base_data: {}",base_data);

    let encode_data = puzzle_encode(base_data); //base_data moved,can't be borrowed at line 19
    println!("encode data: {}",encode_data);
}

fn puzzle_encode(input_data: String) -> String{
    let out_data = input_data;

    let input_len = out_data.len()-1;
    let pieces_count = (input_len+7)/8;
    println!("{} data separate to {} pieces!",input_len,pieces_count);

    for i in 0..(pieces_count-1){
        //let mut picecs_data = String::new();
        let picecs_data = &out_data[i*8..(i+1)*8];
        let test_pac = encode_packet{
            pieces_count : 4,
            pieces_index : i,
            pieces_data : String::from(picecs_data),
        };
        println!("the {} pieces is {:#?}", i,test_pac);
    }
    let last_piece = &out_data[(pieces_count-1)*8..input_len];
    let last_pac = encode_packet{
            pieces_count : 4,
            pieces_index : pieces_count,
            pieces_data : String::from(last_piece),
    };
    println!("the {} pieces is {:#?}", pieces_count,last_pac);
    out_data
}