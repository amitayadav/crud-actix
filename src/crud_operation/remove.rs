use network::*;
use models::Student;
use actix_web::Path;
use cdrs::query::QueryExecutor;

pub fn delete_struct(session: &CurrentSession, id: Path<i32>) {
    let delete_struct_cql = "DELETE FROM student_ks.my_student_table WHERE roll_no = ? ";
    let roll_no = id.into_inner();
    session
        .query_with_values(delete_struct_cql, query_values!(roll_no))
        .expect("delete");
}
