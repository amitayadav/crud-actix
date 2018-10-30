use network::*;
use models::Student;
use actix_web::Json;
use cdrs::query::*;
use cdrs::query::QueryExecutor;
use cdrs::types::prelude::*;

pub fn insert_struct(session: &CurrentSession, new_student: Json<Student>) {
    let stu = Student {
        roll_no: new_student.roll_no,
        marks: new_student.marks,
        name: new_student.name.clone(),

    };
    let insert_struct_cql = "INSERT INTO student_ks.my_student_table \
                           (roll_no ,marks,name) VALUES (?,?,?) ";
    session
        .query_with_values(insert_struct_cql, query_values!( stu.roll_no, stu.marks, stu.name))
        .expect("insert here ");
}
