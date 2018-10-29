use actix_web::{Json, Path, Result/*,HttpRequest*/};
use cdrs::frame::TryFromRow;
use connection::connect;
use db::*;
use super::Student;

pub fn index1(student: Json<Student>) -> Result<String> {
    create_keyspace(&connect());
    create_table(&connect());

    insert_struct(&connect(), student);
    Ok(format!("Welcome ! student added "))
}

pub fn index2(path: Path<i32>) -> Result<Json<Student>> {
    let rows = select_struct(&connect(), path);

    let mut my_row = Student {
        roll_no: 0,
        marks: 0,
        name: "".to_string(),
    };

    for row in rows {
        my_row = Student::try_from_row(row).expect("into Student")
    }
    Ok(Json(Student {
        roll_no: my_row.roll_no,
        marks: my_row.marks,
        name: my_row.name.clone(),
    }))
}

pub fn index3(path: Path<i32>) -> Result<String> {
    delete_struct(&connect(), path);
    Ok(format!(" student deleted "))
}

pub fn index4(student: Json<Student>, path: Path<i32>) -> Result<String> {
    update_struct(&connect(), student, path);
    Ok(format!(" student updated "))
}

