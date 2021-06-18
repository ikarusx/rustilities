//! # Main
//! 
//! This is the entry point of the application,
//! and this is an example for file documentation.
//! It will document the file it is **in**, using *markdown*.

/*! # Obs
 * This is another way to add multiline documentation/lists.
 * 1
 * 2
 * 3
 */

mod samples;
mod tests;

#[allow(unused)]
fn test03_let_mut()
{
    
}

#[allow(unused)]
fn test04_lambdas()
{
    
}

/// # fn main
/// 
/// Main entry-point function.
/// This is an example of documentation
/// to items following it.
/// 
/// You can open/test it with the command:
/// `cargo doc --open`
/// 
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```

fn main()
{
    //samples::s01_types::sample01_structs_and_types();
    samples::s02_control::sample02_ifs_and_loops();
}
