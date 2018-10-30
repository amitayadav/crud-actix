use actix_web::{Json, Path, Result};

use crud_actix::network::connect;
use crud_actix::crud_operations::{insert::*,remove::*,update::*,display::*};
use crud_actix::models::Student;
use crud_actix::set_up::{keyspace::*,
                         table::*};

pub fn insert(student: Json<Student>) -> Result<String> {
    create_keyspace(&connect());
    create_table(&connect());

    insert_struct(&connect(), student);
    Ok(format!("Welcome ! student added "))
}

pub fn show(path: Path<i32>) -> Result<Json<Student>> {
    let student = select_struct(&connect(), path);

    Ok(Json(Student {
        roll_no: student.roll_no,
        marks: student.marks,
        name: student.name.clone(),
    }))
}

pub fn delete(path: Path<i32>) -> Result<String> {
    delete_struct(&connect(), path);
    Ok(format!(" student deleted "))
}

pub fn update(student: Json<Student>, path: Path<i32>) -> Result<String> {
    update_struct(&connect(), student, path);
    Ok(format!(" student updated "))
}

