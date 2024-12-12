use std::fs::{self,File};
use std::io::{self,Write};  //import io si Write pt scriere in fisiere
//use std::hash::{Hash,Hasher};
use std::collections::HashSet;
use sha2::{Sha256,Digest};
//use std::time::{SystemTime,UNIX_EPOCH};
use std::path::Path; //pt manipulare fisiere
fn init_repository() -> io::Result<()>
{
    let dir = Path::new(".vcs");
    //vf daca exista repository
    if dir.exists()
    {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists,"Repository deja initializat.",));
    }
    fs::create_dir(dir)?;
    fs::create_dir(dir.join("branches"))?;
    fs::create_dir(dir.join("commits"))?;
    fs::create_dir(dir.join("staged"))?;

    fs::write(dir.join("HEAD"),"main")?;
    fs::write(dir.join("branches").join("main"),"")?;
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
fn get_current_commit(branch:&str)->io::Result<String>
{
    let branch_path = Path::new(".vcs/branches").join(branch);
    fs::read_to_string(branch_path).map(|s| s.trim().to_string())
}
fn create_branch(branch_name: &str)->io::Result<()>
{
    let branches_dir = Path::new(".vcs").join("branches");
    //vf daca exista deja branch-ul
    let branch_path = branches_dir.join(branch_name);
    if branch_path.exists()
    {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists,format!("Branch-ul '{}' exista deja.",branch_name)));
    }
    fs::create_dir_all(branches_dir)?;
    let current_branch = current_branch()?;
    let tracked_files = get_tracked_files(&current_branch)?;
    let mut branch_file = fs::File::create(&branch_path)?;
    for file in &tracked_files 
    {
        writeln!(branch_file,"{}",file)?;
    }
    let current_commit = get_current_commit(&current_branch)?;
    fs::write(branch_path.with_extension("commit"),current_commit)?;
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
    let head_path = Path::new(".vcs/HEAD");
    fs::write(&head_path,branch_name)?;
    let tracked_files = get_tracked_files(branch_name)?;
    for file in &tracked_files 
    {
        let commit_path = Path::new(".vcs/commits").join(file);
        if commit_path.exists()
        {
            let content = fs::read(&commit_path)?;
            fs::write(file,content)?;
        }
        else {
            if Path::new(file).exists()
            {
                fs::remove_file(file)?;
            }
        }
    }
    println!("Switched pe branch '{}'",branch_name);
    Ok(())

}
fn current_branch()-> io::Result<String>
{
    let dir = Path::new(".vcs").join("HEAD");
    let branch = fs::read_to_string(dir)?.trim().to_string();
    Ok(branch)
}
fn get_tracked_files(branch_name: &str)->io::Result<HashSet<String>>
{
    let dir = Path::new(".vcs").join("branches");
    let branch_path = dir.join(branch_name);
    let mut tracked_files = HashSet::new();

    if branch_path.exists()
    {
        let branch_content = fs::read_to_string(branch_path)?;
        for line in branch_content.lines()
        {
            tracked_files.insert(line.to_string());
        }
    }
    Ok(tracked_files)
}
fn git_status()->io::Result<()>
{
    let branch_name = current_branch()?;
    let tracked_files = get_tracked_files(&branch_name)?;
    println!("On branch: {}",branch_name);

    let mut modified = Vec::new();
    let mut untracked = Vec::new();
    let mut staged=Vec::new();
    
    for entry in fs::read_dir(".")?
    {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.to_str().unwrap().to_string();
        if path.is_file()
        {
            let staged_dir = Path::new(".vcs").join("staged");
            let commit_path = Path::new(".vcs").join("commits").join(&file_name);
            if staged_dir.exists() && staged_dir.join(&file_name).exists()
            {
                staged.push(file_name);
            }
            else 
            {
                if commit_path.exists()
                {
                    let file_content = fs::read(path)?;
                    let commit_content = fs::read(commit_path)?;
                    if file_content != commit_content
                    {
                        modified.push(file_name);
                    }
                }
                else 
                {
                    if !tracked_files.contains(&file_name)
                    {
                        untracked.push(file_name);
                    }
                }
            }
            
        }
    }
    if !modified.is_empty()
    {
        println!("Changes not staged for commit:");
        for file in &modified
        {
            println!(" modified: {}",file);
        }
    }
    if !staged.is_empty()
    {
        println!("Changes to be committed:");
        for file in &staged
        {
            let commit_path = Path::new(".vcs").join("commits").join(file);
            if commit_path.exists()
            {
                println!(" modified: {}",file);
            }
            else
            {
                println!(" new file: {}",file);
            }
        }
    }
    if !untracked.is_empty()
    {
        println!("Untracked files:");
        for file in &untracked
        {
            println!(" {}",file);
        }
    }
    if modified.is_empty() && untracked.is_empty() && staged.is_empty()
    {
        println!("no changes added to commit");
    }
    Ok(())
}
fn git_add(file_name:&str)->io::Result<()>
{
    let dir = Path::new(".vcs");
    let staged_dir = dir.join("staged");
    let file_path = Path::new(file_name);
    if !file_path.exists()
    {
        return Err(io::Error::new(io::ErrorKind::NotFound,format!("Fisierul '{}' nu exista.",file_name)));
    }
    let staged_file_path = staged_dir.join(file_name);
    fs::copy(file_path,staged_file_path)?;
    println!("Fisierul '{}' a fost adaugat la stagiul de commit.",file_name);
    Ok(())
}
fn git_add_all()->io::Result<()>
{
    let dir = Path::new(".vcs");
    let staged_dir = dir.join("staged");
    for entry in fs::read_dir(".")?
    {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.to_str().unwrap().to_string();
        if path.is_file()
        {
            let staged_file_path = staged_dir.join(&file_name);
            if !staged_file_path.exists()
            {
               fs::copy(&path,staged_file_path)?;
            }
        }
    }
    Ok(())

}
fn git_commit(mesaj:&str)->io::Result<()>
{
    let dir = Path::new(".vcs");
    let staged_dir = dir.join("staged");
    let commits_dir = dir.join("commits");
    if !staged_dir.exists() || fs::read_dir(&staged_dir)?.count() == 0
    {
        return Err(io::Error::new(io::ErrorKind::NotFound,"Nu exista fisiere adaugate la stage."));
    }
    let commit_id = generate_commit_id(&staged_dir)?;
    if !commits_dir.exists()
    {
        fs::create_dir_all(&commits_dir)?;
    }
    let commit_file_path = commits_dir.join(commit_id.to_string());
    let mut commit_file = File::create(&commit_file_path)?;
    writeln!(commit_file,"Commit ID: {}",commit_id)?;
    writeln!(commit_file,"Mesaj: {}",mesaj)?;
    let mut tracked_files = HashSet::new();
    for entry in fs::read_dir(&staged_dir)?
    {
        let entry = entry?;
        let path = entry.path();
        if path.is_file()
        {
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            tracked_files.insert(file_name.clone());
            println!("Fisier adaugat in tracked_files: {}",file_name);
            let commit_file_path = commits_dir.join(file_name);
            fs::copy(&path,&commit_file_path)?;
        }
        for file in &tracked_files
        {
            let staged_file_path = staged_dir.join(file);
            println!("Stergere fisier din staged: {:?}",staged_file_path);
            if staged_file_path.exists()
            {
                fs::remove_file(staged_file_path)?;
            }
        }
    }
    println!("Commit realizat cu succes. ID: {}",commit_id);
    Ok(())
    
}
fn generate_commit_id(staged_dir:&Path)->io::Result<String>
{
    let mut hasher = Sha256::new();
    for entry in fs::read_dir(staged_dir)?
    {
        let entry = entry?;
        let path = entry.path();
        if path.is_file()
        {
            let file_content = fs::read(path)?;
            hasher.update(file_content);
        }
    }
    let result = hasher.finalize();
    Ok(format!("{:x}",result))
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
            "git status" =>
            {
                println!("Afisare status repository...");
                match git_status()
                {
                    Ok(_) => {},
                    Err(e) => eprintln!("Eroare: {}",e),
                }
            }
            "git add ." =>
            {
                match git_add_all()
                {
                    Ok(_) => println!("Toate fisierele au fost adaugate cu succes."),
                    Err(e) => eprintln!("Eroare: {}",e),
                }
            }
            comanda if comanda.starts_with("git add ")=>
            {
                if let Some(file_name) = comanda.strip_prefix("git add ")
                {
                    match git_add(file_name)
                    {
                        Ok(_)=> println!("Fisierul '{}' a fost adaugat cu succes.",file_name),
                        Err(e) => eprintln!("Eroare: {}",e),
                    }
                }
            }
            comanda if comanda.starts_with("git commit -m ")=>
            {
                if let Some(mesaj) = comanda.strip_prefix("git commit -m ")
                {
                    let mesaj = mesaj.trim();
                    match git_commit(mesaj)
                    {
                        Ok(_) => println!("Commit realizat cu succes."),
                        Err(e) => eprintln!("Eroare la commit: {}",e),
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
}