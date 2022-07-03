pub mod messages;
pub mod actors;
pub mod executor;
/// 'add_two' 将指定值加2
/// 这是一个测试文档
/// ```
/// let arg = 5;
/// let answer = my_crate::add_two(arg);
///  assert_eq!(7, answer);
/// ```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
