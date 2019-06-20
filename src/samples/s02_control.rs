
#[allow(unused)]
pub fn sample02_ifs_and_loops()
{
    let x = 32;

    let y = if x > 32
        {
            println!("Greater than 32!");
            33
        }
        else if x < 32
        {
            println!("Lesser than 32!");
            31
        }
        else
        {
            println!("It is 32!");
            x
        };

    let mut count = 0;

    'outer: loop
    {
        'inner: loop
        {
            count += 1;

            println!("This is loop #{}!", count);

            if count > 5
            {
                break 'outer;
            }
        }
    }

    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() { // into_iter -> consume / iter_mut -> modify
        match name { // match =~ switch-case
            // "Ferris" / &mut "Ferris"
            &"Ferris" => println!("There is a rustacean among us!"),
            // default / wildcard
            _ => println!("Hello {}", name),
        }
    }
}