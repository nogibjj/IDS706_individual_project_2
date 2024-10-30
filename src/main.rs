use rusqlite::{params, Connection, Result};

// Function to connect to the SQLite database
pub fn connect_to_db(db_name: &str) -> Result<Connection> {
    let conn = Connection::open(db_name)?;
    Ok(conn)
}

// Function to create the grades table with a composite primary key
pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS grades (
            duke_id INTEGER NOT NULL,
            assignment_id INTEGER NOT NULL,
            grade REAL NOT NULL,
            PRIMARY KEY (duke_id, assignment_id)
        );
        ",
        [],
    )?;
    Ok(())
}

// Function to insert data into the grades table
pub fn insert_data(conn: &Connection, duke_id: i32, assignment_id: i32, grade: f32) -> Result<()> {
    conn.execute(
        "INSERT INTO grades (duke_id, assignment_id, grade) VALUES (?1, ?2, ?3)",
        params![duke_id, assignment_id, grade],
    )?;
    Ok(())
}

// Function to read all data from the grades table
pub fn read_data(conn: &Connection) -> Result<Vec<(i32, i32, f32)>> {
    let mut stmt = conn.prepare("SELECT duke_id, assignment_id, grade FROM grades")?;
    let grades_iter = stmt.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    })?;

    let mut grades = Vec::new();
    for grade in grades_iter {
        grades.push(grade?);
    }
    Ok(grades)
}

// Function to update a grade
pub fn update_data(conn: &Connection, duke_id: i32, assignment_id: i32, new_grade: f32) -> Result<()> {
    conn.execute(
        "UPDATE grades SET grade = ?1 WHERE duke_id = ?2 AND assignment_id = ?3",
        params![new_grade, duke_id, assignment_id],
    )?;
    Ok(())
}

// Function to delete a grade entry
pub fn delete_data(conn: &Connection, duke_id: i32, assignment_id: i32) -> Result<()> {
    conn.execute(
        "DELETE FROM grades WHERE duke_id = ?1 AND assignment_id = ?2",
        params![duke_id, assignment_id],
    )?;
    Ok(())
}

// Function to get the average grade per student
pub fn get_average_grade_per_student(conn: &Connection) -> Result<Vec<(i32, f32)>> {
    let mut stmt = conn.prepare(
        "SELECT duke_id, AVG(grade) as average_grade FROM grades GROUP BY duke_id",
    )?;
    let avg_iter = stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, f32>(1)?))
    })?;

    let mut averages = Vec::new();
    for avg in avg_iter {
        averages.push(avg?);
    }
    Ok(averages)
}

// Function to get high achievers (grades > 90)
pub fn get_high_achievers(conn: &Connection) -> Result<Vec<(i32, i32)>> {
    let mut stmt = conn.prepare(
        "SELECT duke_id, assignment_id FROM grades WHERE grade > 90",
    )?;
    let achievers_iter = stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?))
    })?;

    let mut achievers = Vec::new();
    for achiever in achievers_iter {
        achievers.push(achiever?);
    }
    Ok(achievers)
}

fn main() -> Result<()> {
    // Connect to the database
    let conn = connect_to_db("ids706_grades.db")?;

    // Create the table with composite primary key (duke_id, assignment_id)
    create_table(&conn)?;

    // Insert some sample data
    insert_data(&conn, 1234567, 0, 85.0)?;  // Assignment ID starts from 0
    insert_data(&conn, 1234567, 1, 90.5)?;  // Next assignment ID
    insert_data(&conn, 1234567, 2, 99.0)?;  // Assignment ID 2
    insert_data(&conn, 1234567, 3, 94.0)?;  // Assignment ID 3
    insert_data(&conn, 7654321, 0, 78.0)?;  // Assignment ID 0 for another student
    insert_data(&conn, 7654321, 1, 85.0)?;  // Assignment ID 1 for another student
    insert_data(&conn, 7654321, 2, 83.0)?;  // Assignment ID 2 for another student

    println!("Data after insertion:");
    view_data(&conn)?;

    // Update a grade
    update_data(&conn, 1234567, 0, 88.0)?;  // Update assignment 0 for duke_id 1234567
    println!("\nData after update:");
    view_data(&conn)?;

    // Delete a grade
    delete_data(&conn, 7654321, 0)?;  // Delete assignment 0 for duke_id 7654321
    println!("\nData after deletion:");
    view_data(&conn)?;

    // Get average grade per student
    println!("\nAverage Grade per Student:");
    let averages = get_average_grade_per_student(&conn)?;
    for (duke_id, average_grade) in averages {
        println!("{:<7} | {:.3}", duke_id, average_grade);
    }

    // Get high achievers
    println!("\nHigh Achievers (Grades > 90):");
    let achievers = get_high_achievers(&conn)?;
    for (duke_id, assignment_id) in achievers {
        println!("{}    | {}", duke_id, assignment_id);
    }

    Ok(())
}

// Function to display all grades
pub fn view_data(conn: &Connection) -> Result<()> {
    let rows = read_data(conn)?;

    println!("Duke ID | Assignment ID | Grade");
    println!("-------------------------------");
    for row in rows {
        println!("{:<7} | {:<12} | {:.1}", row.0, row.1, row.2);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() -> Result<()> {
        let conn = connect_to_db(":memory:")?;

        create_table(&conn)?;

        insert_data(&conn, 1234567, 0, 85.0)?;

        let data = read_data(&conn)?;
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].0, 1234567);
        assert_eq!(data[0].1, 0);
        assert_eq!(data[0].2, 85.0);

        Ok(())
    }

    #[test]
    fn test_read() -> Result<()> {
        let conn = connect_to_db(":memory:")?;

        create_table(&conn)?;

        insert_data(&conn, 1234567, 0, 85.0)?;
        insert_data(&conn, 1234567, 1, 90.5)?;

        let data = read_data(&conn)?;
        assert_eq!(data.len(), 2);

        assert_eq!(data[0].0, 1234567);
        assert_eq!(data[0].1, 0);
        assert_eq!(data[0].2, 85.0);

        assert_eq!(data[1].0, 1234567);
        assert_eq!(data[1].1, 1);
        assert_eq!(data[1].2, 90.5);

        Ok(())
    }

    #[test]
    fn test_update() -> Result<()> {
        let conn = connect_to_db(":memory:")?;

        create_table(&conn)?;

        insert_data(&conn, 1234567, 0, 85.0)?;

        update_data(&conn, 1234567, 0, 88.0)?;

        let data = read_data(&conn)?;
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].2, 88.0);

        Ok(())
    }

    #[test]
    fn test_delete() -> Result<()> {
        let conn = connect_to_db(":memory:")?;

        create_table(&conn)?;

        insert_data(&conn, 1234567, 0, 85.0)?;
        insert_data(&conn, 1234567, 1, 90.5)?;

        delete_data(&conn, 1234567, 0)?;

        let data = read_data(&conn)?;
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].1, 1);
        assert_eq!(data[0].2, 90.5);

        Ok(())
    }

    #[test]
    fn test_get_average_grade_per_student() -> Result<()> {
        let conn = connect_to_db(":memory:")?;

        create_table(&conn)?;

        // Insert sample data
        insert_data(&conn, 1234567, 0, 85.0)?;
        insert_data(&conn, 1234567, 1, 90.5)?;
        insert_data(&conn, 7654321, 0, 78.0)?;
        insert_data(&conn, 7654321, 1, 85.0)?;

        let averages = get_average_grade_per_student(&conn)?;
        assert_eq!(averages.len(), 2);

        // Since SQL GROUP BY does not guarantee order, we need to check both entries
        for (duke_id, average_grade) in averages {
            if duke_id == 1234567 {
                assert!((average_grade - 87.75).abs() < f32::EPSILON);
            } else if duke_id == 7654321 {
                assert!((average_grade - 81.5).abs() < f32::EPSILON);
            } else {
                panic!("Unexpected duke_id: {}", duke_id);
            }
        }

        Ok(())
    }

    #[test]
    fn test_get_high_achievers() -> Result<()> {
        let conn = connect_to_db(":memory:")?;

        create_table(&conn)?;

        insert_data(&conn, 1234567, 0, 85.0)?;
        insert_data(&conn, 1234567, 1, 91.0)?;
        insert_data(&conn, 1234567, 2, 95.0)?;
        insert_data(&conn, 7654321, 0, 78.0)?;
        insert_data(&conn, 7654321, 1, 85.0)?;

        let achievers = get_high_achievers(&conn)?;
        assert_eq!(achievers.len(), 2);

        assert!(achievers.contains(&(1234567, 1)));
        assert!(achievers.contains(&(1234567, 2)));

        Ok(())
    }
}
