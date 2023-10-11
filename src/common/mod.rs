/**
格式化输出请求体
*/
pub fn format_request_body(body: &[u8]) {
    println!("{:02X?}", body)
}
