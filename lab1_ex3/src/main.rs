fn main() {
   
   //3.
   let mut bottles = 99;
   loop {
       println!("{bottles} bottles of beer on the wall,");
       println!("{bottles} bottles of beer.");
       println!("Take one down, pass it around,");
       bottles-=1;
       println!("{bottles} bottles of beer on the wall.\n");

       if bottles == 1
       {
            println!("{bottles} bottle of beer on the wall,");
            println!("{bottles} bottle of beer.");
            println!("Take one down, pass it around,");
            println!("No bottles of beer on the wall.");
            break;
       }
   }
}
