
struct MyStruct
{
    my_int: i32,
    my_uint: u32,
    my_char: char,
}

impl MyStruct
{
    pub fn initialize() -> Self
    {
        println!("Initializing MyStruct ...");
        let my_int = -333;
        let my_uint = 333;
        let my_char = '†';

        Self
        {
            my_int,
            my_uint,
            my_char
        }
    }

    pub fn initialize_with_values(
        my_int : i32,
        my_uint : u32,
        my_char : char) -> Self
    {
        println!("Initializing MyStruct ...");

        Self
        {
            my_int,
            my_uint,
            my_char
        }
    }

    fn print_members(&self)
    {
        println!("\nMembers are:");
        println!("\tmy_int: {}", self.my_int);
        println!("\tmy_uint: {}", self.my_uint);
        println!("\tmy_char: {}\n", self.my_char);
    }
}

#[allow(unused)]
pub fn sample01_structs_and_types()
{
    println!("Hello, world!");
    let my_struct = MyStruct::initialize();
    my_struct.print_members();
    let my_struct2 = MyStruct::initialize_with_values(
        -1024, 1024, 'æ');
    my_struct2.print_members();
}