
use postgres::{error::Error,Client,NoTls};

#[derive(Debug)]
struct Person {
    id : i32,
    name:String,
    data:Option<Vec<u8>>
}

fn create_db() -> Result<Client,Error> {
    let user_name = "postgres";
    let password = "1234";
    let host = "tccl";
    let port = "5432";
    let database = "postgres";

    let conn_str = &format!(
        "postgres://{}{}{}@{}{}{}{}{}",
        user_name,
        if password.is_empty() {""} else {":"},
        password,
        host,
        if port.is_empty() {""} else {":"},
        port,
        if database.is_empty() {""} else {"/"},
        database
    );

    let client  = Client::connect(conn_str,NoTls)?;

    // let _ = client.execute("drop table person",&[]);
    // client.execute(
    //     "create table person (
    //         id serial primary key,
    //         name text not null ,
    //         data bytea
    //     )",&[]
    // )?;
    Ok(client)
}

fn insert_data(client:&mut Client) -> Result<(),Error> {
    let p1 = Person {
        id:1,
        name:"Dave".to_string(),
        data:None,
    };
    let p2 = Person {
        id :2 ,
        name :"Nick".to_string(),
        data:None
    };

    client.execute(
        "insert into person (id,name,data) 
        values ($1,$2,$3),
        ($4,$5,$6)",
        &[&p1.id,&p1.name,&p1.data,
          &p2.id,&p2.name,&p2.data]
    )?;
    Ok(())
}

fn get_data(client:&mut Client) -> Result<Vec<Person>,Error> {
    let mut persons = Vec::new();
    for row in client.query("select id,name ,data from person",&[])?{
        persons.push(Person {
            id:row.get(0),
            name:row.get(1),
            data:row.get(2)
        });
    }
    Ok(persons)
}


pub fn db_test()-> Result<(),Error> {

    let mut client = create_db()?;

    // insert_data(&mut client)?;
    let persons = get_data(&mut client)?;
    for p in persons {
        println!("Person: {:?}",p);
    }

    Ok(())
}