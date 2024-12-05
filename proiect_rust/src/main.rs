use std::fs;
use std::io::{self,Write};  //import io si Write pt scriere in fisiere
use std::path::Path; //pt manipulare fisiere
fn init_repository() -> io::Result<()>
{
    // whatever
    let dir = Path::new(".vcs");
    //vf daca exista repository
    if dir.exists()
    {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists,"Repository deja initializat.",));
    }
    fs::create_dir(dir)?;
    fs::create_dir(dir.join("branches"))?;
    fs::create_dir(dir.join("commits"))?;

    fs::write(dir.join("HEAD"),"main")?;
    fs::write(dir.join("branches").join("main"),"")?;
    fs::write(dir.join("staged"),"")?;
    Ok(())
}
fn show_branches()->io::Result<()>
{
    let dir = Path::new(".vcs").join("branches");
    if !dir.exists()
    {
        return Err(io::Error::new(io::ErrorKind::NotFound,"Nu exista branch-uri."));
    }
    let fis = fs::read_dir(dir)?;
    let mut branches = Vec::new();
    for fisier in fis 
    {
        let fisier = fisier?;
        let file_name = fisier.file_name();
        if let Some(name) = file_name.to_str()
        {
            branches.push(name.to_string());
        }
    }
    if branches.is_empty()
    {
        println!("Nu exista branch-uri!");
    }
    else {
        println!("Branch-uri existente: ");
        for branch in branches 
        {
            println!("{}",branch);
        }
    }
    Ok(())
}
fn create_branch(branch_name: &str)->io::Result<()>
{
    let dir = Path::new(".vcs").join("branches");
    //vf daca exista deja branch-ul
    let branch_path = dir.join(branch_name);
    if branch_path.exists()
    {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists,format!("Branch-ul '{}' exista deja.",branch_name)));
    }
    fs::write(branch_path,"")?;
    println!("Branch-ul '{}' a fost creat cu succes.",branch_name);
    Ok(())
}
fn checkout_branch(branch_name:&str)->io::Result<()>
{
    let dir = Path::new(".vcs");
    let branch_dir = dir.join("branches");
    //vf daca exista branch
    let branch_path = branch_dir.join(branch_name);
    if !branch_path.exists()
    {
        return Err(io::Error::new(io::ErrorKind::NotFound,format!("Nu exista branchul '{}'.",branch_name)));
    }
    fs::write(dir.join("HEAD"),branch_name)?;
    println!("Checkout pe branch '{}'",branch_name);
    Ok(())

}
fn commit_id(branch_name: &str)->io::Result<u64>
{
    let dir = Path::new(".vcs");
    let cnt_file = dir.join("branches").join(format!("{}.counter",branch_name));
    if cnt_file.exists()
    {
        let cnt = fs::read_to_string(cnt_file)?
        .trim()
        .parse::<u64>()
        .unwrap_or(0);
        Ok(cnt)
    }
    else {
        Ok(0)
    }
}
fn increment_commit(branch_name: &str)->io::Result<()>
{
    let dir = Path::new(".vcs");
    let cnt_file = dir.join("branches").join(format!("{}.counter",branch_name));
    let current_id = commit_id(branch_name)?;
    let new_id = current_id + 1;
    //scriem contorul in fisier
    fs::write(cnt_file,new_id.to_string())?;
    Ok(())
}
fn commit_changes(mesaj: &str)->io::Result<()>
{
    let dir = Path::new(".vcs");
    let commit_dir = dir.join("commits");
    let branch_dir = dir.join("branches");
    let head_file = dir.join("HEAD");

    //vedem in ce branch suntem
    let current_branch = fs::read_to_string(head_file)?.trim().to_string();
    //vf daca exista branch-ul curent
    let branch_path = branch_dir.join(&current_branch);
    if !branch_path.exists()
    {
        return Err(io::Error::new(io::ErrorKind::NotFound,format!("Branchul '{}' nu exista",current_branch)));
    }
    //obtinem id pt commit curent
    let commit_id = commit_id(&current_branch)?;
    //cream fisierul de commit cu mesajul
    let commit_path = commit_dir.join(format!("commit-{}-{}",current_branch,commit_id+1));
    let commit_info = format!("Id commit: {}\nBranch: {}\n Mesaj: {}\n",commit_id+1,current_branch,mesaj);
    fs::write(commit_path,commit_info)?;
    fs::write(branch_path,&(commit_id+1).to_string())?;
    increment_commit(&current_branch)?;
    println!("Commit realizat cu succes. Id commit: {}",commit_id+1);
    Ok(())
}
fn current_branch()-> io::Result<String>
{
    let dir = Path::new(".vcs").join("HEAD");
    let branch = fs::read_to_string(dir)?.trim().to_string();
    Ok(branch)
}
fn git_status() -> io::Result<()>
{
    let dir = Path::new(".vcs");
    if !dir.exists()
    {
        return Err(io::Error::new(io::ErrorKind::NotFound,"Repository-ul nu a fost initializat.",));
    }
    //citesc 
    let head_file = dir.join("HEAD");
    let branch_file = dir.join("branches");
    let staged_file = dir.join("staged");
    let current_branch = fs::read_to_string(&head_file)?.trim().to_string();
    println!("Sunteti pe branch-ul: {}",current_branch);

    Ok(())
}
fn main() 
{
    println!("Bun venit in aplicatia my_svn!");

    loop {
        //afisam promptul pentru user
        print!("vcs> ");
        io::stdout().flush().unwrap();

        //citim comanda introdusa de user
        let mut comanda = String::new();
        io::stdin().read_line(&mut comanda).unwrap();
        let comanda = comanda.trim(); //eliminam spatiile
        match comanda
        {
            "git init" => {
                println!("Initializare repository...");
                match init_repository()
                {
                    Ok(_) => println!("Repository initializat cu succes!"),
                    Err(e) => eprintln!("Eroare {}",e),
                }
            }
            "git branch --show-current"=>
            {
                match current_branch()
                {
                    Ok(branch) => println!("Te afli pe branch-ul: {}",branch),
                    Err(e) => eprintln!("Eroare: {}",e),
                }
            }
            comanda if comanda.starts_with("git checkout ") =>
            {
                if let Some(name) = comanda.strip_prefix("git checkout ")
                {
                    println!("Comanda de checkout pe branchul '{}'",name);
                    match checkout_branch(name)
                    {
                        Ok(_) => println!("Switch pe branchul '{}'",name),
                        Err(e) => eprintln!("Eroare: {}",e),
                    }
                }
                else {
                    eprintln!("Comanda 'git checkout' invalida.");
                }
            }
            comanda if comanda == "git branch" => {
                match show_branches()
                {
                    Ok(_) => {},
                    Err(e) => eprintln!("Eroare: {}",e),
                }
            }
            comanda if comanda.starts_with("git branch ")=> {
                if let Some(name) = comanda.strip_prefix("git branch ")
                {
                    match create_branch(name)
                    {
                        Ok(_) => {},
                        Err(e)=> eprintln!("Eroare: {}",e),
                    }
                }
            }
            comanda if comanda.starts_with("git commit -m ") => {
                if let Some(mesaj) = comanda.strip_prefix("git commit -m ")
                {
                    match commit_changes(mesaj)
                    {
                        Ok(_) => {},
                        Err(e) => eprintln!("Eroare: {}",e),
                    }
                }
            }
            "quit" => {
                println!("Ai iesit din aplicatia my_svn! Te asteptam data viitoare.");
                break;
            }
            _ =>
            {
                println!("Comanda necunoscuta: {}",comanda);
                println!("Comenzile disponibile sunt: git init : initializeaza repository\ngit checkout <nume_branch> : te muta pe alt branch\ngit branch : afiseaza lista de branch-uri\ngit branch <nume_branch> : creeaza un nou branch cu numele nume_branch\n quit : iesi din aplicatie\n");
            }
        }
    }
    //git branch --show-current
    //git add <path>
    //git ignore - fisiere pe care nu le urc (fisiere ignorate) - eu am fisierul in ierarhie 
    //loc in care 
}