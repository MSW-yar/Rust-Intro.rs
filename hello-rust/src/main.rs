/** fn main() {
*   println!("Hello, world!");
* }
*/

use ferris_says:: say;
// this tells that we can use say that ferris_say is providing.

use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let out = b"Hello baby";
    let width = 14;
    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
}
