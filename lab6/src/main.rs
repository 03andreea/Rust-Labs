//p1
use std::{io,fs};
trait Command
{
    fn get_name(&self)->&'static str;
    fn exec(&mut self,args:&[&str]);
}
struct Ping;
impl Command for Ping
{
    fn get_name(&self)->&'static str
    {
        "ping"
    }
    fn exec(&mut self,_args:&[&str])
    {
        println!("pong!");
    }
}
struct Count;
impl Command for Count
{
    fn get_name(&self)->&'static str
    {
        "count"
    }
    fn exec(&mut self,args:&[&str])
    {
        println!("Comanda are {} argumente",args.len());
    }
}
struct Times
{
    counter: u32,
}
impl Times
{
    fn new()->Self
    {
        Times { counter: 0}
    }
}
impl Command for Times
{
    fn get_name(&self)->&'static str
    {
        "times"
    }
    fn exec(&mut self,_args:&[&str])
    {
        self.counter+=1;
        println!("Comanda times a fost executata de {} ori",self.counter);
    }
}
#[derive(Debug)]
enum TerminalError
{
    InvalidCommand(String),
    StopExecution,
    Suggestion,
}
struct Terminal
{
    comenzi : Vec<Box<dyn Command>>,
    stop : bool,
}
impl Terminal
{
    fn new()->Self
    {
        Terminal 
        { 
            comenzi:Vec::new(),
            stop:false, 
        }
    }
    fn register(&mut self,comanda:Box<dyn Command>)
    {
        self.comenzi.push(comanda);
    }
    fn run(&mut self,file_path:&str)->Result<(),io::Error>
    {
        let s = fs::read_to_string(file_path)?;
        for linie in s.lines()
        {
            if self.stop
            {
                break;
            }
            let parti: Vec<&str> = linie.trim().split_whitespace().collect();
            if parti.is_empty()
            {
                continue;
            }
            let command_name=parti[0];
            let args=&parti[1..];
            match self.execute_command(&command_name,args)
            {
                Ok(_)=> (),
                Err(TerminalError::InvalidCommand(cmd))=>{
                    println!("Nu este valida comanda '{}'!",cmd);
                },
                Err(TerminalError::StopExecution) => {
                    println!("Executie oprita!");
                    break;
                }
                Err(TerminalError::Suggestion)=>
                {
                    println!("Comanda valida este '{}'. Foloseste litere mici pentru a scrie comanda!",command_name.to_lowercase());
                }
            }

        }
        Ok(())
    }
    fn execute_command(&mut self,command_name:&str,args:&[&str])->Result<(),TerminalError>
    {
        let command_name_lower = command_name.to_lowercase();
        if command_name_lower == "stop"
        {
            self.stop = true;
            return Err(TerminalError::StopExecution);
        }
        for command in self.comenzi.iter_mut()
        {
            if command.get_name().to_lowercase() == command_name_lower
            {
                if command.get_name() != command_name
                {
                    return Err(TerminalError::Suggestion);
                }
                command.exec(args);
                return Ok(());
            }
        }
       Err(TerminalError::InvalidCommand(command_name.to_string()))
    }
}
fn main_ex1() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(Ping));
    terminal.register(Box::new(Count));
    terminal.register(Box::new(Times::new()));

    if let Err(e) = terminal.run("D:/teo/ANUL 2/RUST/RUST_Andreea_Arama_2B3/lab6/commands.txt")
    {
        eprintln!("Eroare {:?}",e);
    }
}

//p2
use rusqlite::{Connection,Result};
struct Bookmark
{
    name : String,
    url : String,
}
fn initializare_database(conn: &Connection)->Result<()>
{
    let create_table = r"
    CREATE TABLE IF NOT EXISTS bookmarks (
        name TEXT NOT NULL,
        url TEXT NOT NULL);
        ";
    conn.execute(create_table,())?;
    Ok(())
}
fn add_bookmark(conn:&Connection,name:&str,url:&str)->Result<()>
{
    conn.execute("INSERT INTO bookmarks (name,url) values (?1,?2);",(name,url))?;
    println!("Bookmark '{}' adaugat cu succes.",name);
    Ok(())
}
fn search_bookmark(conn:&Connection,searched:&str)->Result<()>
{
    let mut stmt = conn.prepare("SELECT distinct name,url FROM bookmarks WHERE  name LIKE ?1")?;
    let pattern = format!("%{}%", searched);
    let bookmark_iter = stmt.query_map([pattern],|row| {
        Ok(Bookmark {
            name: row.get(0)?,
            url: row.get(1)?,
        })
    })?;
    for bookmark in bookmark_iter
    {
        let bookmark = bookmark?;
        println!("Nume: {}, URL: {}", bookmark.name,bookmark.url);
    }
    Ok(())
}
fn bk(conn: &Connection,args:Vec<String>)->Result<()>
{
    match args[1].as_str()
    {
        "add" => {
            if args.len() != 4
            {
                println!("Comanda arata asa : bk add <name> <url>");
                return Ok(());
            }
            let name = &args[2];
            let url = &args[3];
            add_bookmark(conn,name,url)?;
        }
        "search" => {
            if args.len() != 3
            {
                println!("Comanda arata asa: bk search <name>");
                return Ok(());
            }
            let searched=&args[2];
            search_bookmark(conn,searched)?;
        }
        _ => {
            println!("Comanda trebuie sa arate asa: bk <add|search> <name> <url>");
        }

    }
    Ok(())
}
fn main_ex2()->Result<(),Box<dyn std::error::Error>>
{
    let conn = Connection::open("bookmarks.db")?;
    initializare_database(&conn)?;
    let s = fs::read_to_string("D:/teo/ANUL 2/RUST/RUST_Andreea_Arama_2B3/lab6/bk.txt")?;
    for line in s.lines()
    {
        let mut args = Vec::new();
        for cuv in line.trim().split_whitespace()
        {
            args.push(cuv.to_string());
        }
        if args.is_empty()
        {
            continue;
        }
        if args[0] == "bk"
        {
            bk(&conn,args)?;
        }

    }
    Ok(())
}
fn main()
{
    println!("----------------------------------ex1------------------------------------");
    main_ex1();
    println!("----------------------------------ex2------------------------------------");
    if let Err(e) = main_ex2()
    {
        eprintln!("Avem eroarea {e}");
    }
}
