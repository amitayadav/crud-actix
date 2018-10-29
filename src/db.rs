use actix_web::{Json, Path};
use cdrs::query::*;
use cdrs::types::prelude::*;
use connection::CurrentSession;
use super::Student;

pub fn create_keyspace(session: &CurrentSession) {
    let create_ks: &'static str = "CREATE KEYSPACE IF NOT EXISTS student_ks WITH REPLICATION = { \
                                 'class' : 'SimpleStrategy', 'replication_factor' : 1 };";
    session
        .query(create_ks)
        .expect("keyspace creation error");
}

pub fn create_table(session: &CurrentSession) {
    let create_table_cql =
        "CREATE TABLE IF NOT EXISTS student_ks.my_student_table (roll_no int PRIMARY KEY , \
     name text, marks int);";
    session
        .query(create_table_cql)
        .expect("Table creation error");
}

pub fn insert_struct(session: &CurrentSession, new_student: Json<Student>) {
    let stu = Student {
        roll_no: new_student.roll_no,
        marks: new_student.marks,
        name: new_student.name.clone(),

    };
    let insert_struct_cql = "INSERT INTO student_ks.my_student_table \
                           (roll_no ,marks,name) VALUES (?,?,?) ";
    session
        .query_with_values(insert_struct_cql, stu.into_query_values())
        .expect("insert here ");
}

pub fn select_struct(session: &CurrentSession, path: Path<i32>) -> Vec<Row> {
    let select_struct_cql = "SELECT * FROM student_ks.my_student_table where roll_no = ?";
    let roll_no = path.into_inner();

    let rows = session.query_with_values(select_struct_cql, query_values!(roll_no))
        .expect("update")
        .get_body()
        .expect("get body")
        .into_rows()
        .expect("into rows");
    rows
}

pub fn update_struct(session: &CurrentSession, new_student: Json<Student>, path: Path<i32>) {
    let update_struct_cql = "UPDATE student_ks.my_student_table SET  marks=?,name=? WHERE roll_no = ? If exists ";
    let stu: Student = Student {
        roll_no: path.into_inner(),
        marks: new_student.marks,
        name: new_student.name.clone(),
    };
    session
        .query_with_values(update_struct_cql, query_values!(stu.marks,stu.name,stu.roll_no))
        .expect("update");
}

pub fn delete_struct(session: &CurrentSession, id: Path<i32>) {
    let delete_struct_cql = "DELETE FROM student_ks.my_student_table WHERE roll_no = ? ";
    let roll_no = id.into_inner();
    session
        .query_with_values(delete_struct_cql, query_values!(roll_no))
        .expect("delete");
}

