pub mod mountain;
pub mod surrounding_mountain;

pub fn invalid_param_error(query_name: &str) -> String {
    format!("クエリパラメータ {} の値が不正です。", query_name)
}
