use webthing_cli::get_things;

fn main() -> anyhow::Result<()> {
    let things = get_things(1)?;

    println!("{:?}", things);

    Ok(())
}
